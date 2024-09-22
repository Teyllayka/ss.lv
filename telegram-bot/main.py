from datetime import datetime
import os
from dotenv import load_dotenv
import telebot
from telebot.types import InlineKeyboardMarkup, InlineKeyboardButton, ReplyKeyboardMarkup, KeyboardButton
from sqlalchemy.orm import Session
from database import SessionLocal
import models
from sqlalchemy.exc import IntegrityError


load_dotenv()

BOT_TOKEN = os.getenv("BOT_TOKEN")

if not BOT_TOKEN:
    raise ValueError("BOT_TOKEN is not set in the environment variables.")

bot = telebot.TeleBot(BOT_TOKEN)

user_state = {}  

db: Session = SessionLocal()


@bot.message_handler(commands=['start', 'help'])
def send_welcome(message: telebot.types.Message):
    chat_id = message.chat.id
    try:
        user = db.query(models.User).filter(models.User.telegram_id == str(chat_id)).first()
        if user:
            keyboard = InlineKeyboardMarkup()
            keyboard.add(InlineKeyboardButton('Create Advert', callback_data='create_advert'))

            bot.send_message(chat_id, 
                             f"Welcome back, {user.name} {user.surname}!\n"
                             f"Phone: {user.phone}\n"
                             f"Balance: {user.balance}\n"
                             f"Active adverts: {len(user.adverts)}\n", reply_markup=keyboard
                             )
        else:
            keyboard = InlineKeyboardMarkup()
            keyboard.add(InlineKeyboardButton('Register', callback_data='register'))
            keyboard.add(InlineKeyboardButton('Button 2', callback_data='button2'))

            bot.send_message(chat_id, 'Please choose an option:', reply_markup=keyboard)
    
    except Exception as e:
        bot.send_message(chat_id, "An error occurred. Please try again later.")
        print(f"Error querying user: {e}")

@bot.callback_query_handler(func=lambda call: call.data == 'create_advert')
def handle_create_advert(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id)
    chat_id = call.message.chat.id

    # Initialize advert creation state for the user
    user_state[chat_id] = {
        'step': 'price',
        'advert_data': {}
    }

    # Ask for the price first
    bot.send_message(chat_id, "Let's create an advert! Please provide the price (in numbers):")

# Handler for user input during advert creation
@bot.message_handler(func=lambda message: message.chat.id in user_state and user_state[message.chat.id]['step'] in ['price', 'location', 'title', 'description', 'category', 'photos'])
def handle_advert_creation(message):
    chat_id = message.chat.id
    user_info = user_state[chat_id]
    
    # Determine the current step
    current_step = user_info['step']

    # Collect the data based on the step and store it in user_state
    if current_step == 'price':
        try:
            price = float(message.text)  # Convert input to float for price
            user_info['advert_data']['price'] = price
            user_info['step'] = 'location'
            bot.send_message(chat_id, "Please provide the location:")
        except ValueError:
            bot.send_message(chat_id, "Invalid price. Please enter a number.")
            return

    elif current_step == 'location':
        user_info['advert_data']['location'] = message.text
        user_info['step'] = 'title'
        bot.send_message(chat_id, "Please provide the advert title:")

    elif current_step == 'title':
        user_info['advert_data']['title'] = message.text
        user_info['step'] = 'description'
        bot.send_message(chat_id, "Please provide a description:")

    elif current_step == 'description':
        user_info['advert_data']['description'] = message.text
        user_info['step'] = 'category'
        bot.send_message(chat_id, "Please provide a category:")

    elif current_step == 'category':
        user_info['advert_data']['category'] = message.text
        user_info['step'] = 'photos'
        bot.send_message(chat_id, "Please provide a comma-separated list of photo URLs (at least one):")

    elif current_step == 'photos':
        photos = message.text.split(",")  # Expect a comma-separated list of URLs
        user_info['advert_data']['photos'] = [photo.strip() for photo in photos]
        
        # Proceed to the final step: confirm and submit
        confirm_advert(chat_id)

# Function to confirm advert details before submission
def confirm_advert(chat_id):
    user_info = user_state[chat_id]['advert_data']

    # Display advert summary to the user for confirmation
    summary = (f"Please confirm your advert details:\n"
               f"Price: {user_info['price']}\n"
               f"Location: {user_info['location']}\n"
               f"Title: {user_info['title']}\n"
               f"Description: {user_info['description']}\n"
               f"Category: {user_info['category']}\n"
               f"Photos: {', '.join(user_info['photos'])}")

    confirm_keyboard = InlineKeyboardMarkup()
    confirm_keyboard.add(InlineKeyboardButton('Confirm', callback_data='confirm_advert'))
    confirm_keyboard.add(InlineKeyboardButton('Edit', callback_data='edit_advert'))

    bot.send_message(chat_id, summary, reply_markup=confirm_keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'confirm_advert')
def handle_confirm_advert(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id)
    chat_id = call.message.chat.id

    advert_data = user_state[chat_id]['advert_data']

    try:
        user = db.query(models.User).filter(models.User.telegram_id == str(chat_id)).first()

        if user:
            new_advert = models.Advert(
                title=advert_data['title'],
                description=advert_data['description'],
                price=advert_data['price'],
                location=advert_data['location'],
                category=advert_data['category'],
                photo_url=advert_data['photos'][0],  
                additional_photos=advert_data['photos'][1:],  
                created_at=datetime.utcnow(),
                updated_at=datetime.utcnow(),
                user_id=user.id  
            )

            db.add(new_advert)
            db.commit()
            db.refresh(new_advert)

            bot.send_message(chat_id, "Advert created successfully!")
        
        else:
            bot.send_message(chat_id, "User not found. Please register first.")
    
    except Exception as e:
        db.rollback()
        print(f"Error during advert creation: {e}")
        bot.send_message(chat_id, "An error occurred while creating the advert. Please try again later.")
    
    finally:
        del user_state[chat_id]


# Callback handler for advert editing
@bot.callback_query_handler(func=lambda call: call.data == 'edit_advert')
def handle_edit_advert(call):
    bot.answer_callback_query(call.id)
    chat_id = call.message.chat.id
    bot.send_message(chat_id, "Please specify which detail you'd like to edit (price, location, title, description, category, photos).")



@bot.callback_query_handler(func=lambda call: call.data == 'register')
def handle_register(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id) 
    keyboard = InlineKeyboardMarkup()
    keyboard.add(InlineKeyboardButton('New User', callback_data='new_user'))
    keyboard.add(InlineKeyboardButton('Connect Account', callback_data='connect_account'))

    bot.edit_message_text(chat_id=call.message.chat.id, message_id=call.message.message_id,
                          text="You pressed the Register button!", reply_markup=keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'new_user')
def start_registration(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id)  
    chat_id = call.message.chat.id
    user_state[chat_id] = {'step': 'name'} 
    contact_keyboard = ReplyKeyboardMarkup(one_time_keyboard=True, resize_keyboard=True)
    contact_keyboard.add(KeyboardButton('Share Contact', request_contact=True))

    bot.send_message(chat_id, "Please share your contact information:", reply_markup=contact_keyboard)

@bot.message_handler(content_types=['contact'])
def handle_contact(message: telebot.types.Message):
    chat_id = message.chat.id

    if message.contact:
        first_name = message.contact.first_name
        last_name = message.contact.last_name if message.contact.last_name else ""
        phone_number = message.contact.phone_number

        user_state[chat_id] = {'name': first_name, 'surname': last_name, 'phone': phone_number, 'step': 'confirm'}

        confirm_keyboard = InlineKeyboardMarkup()
        confirm_keyboard.add(InlineKeyboardButton('Confirm', callback_data='confirm'))
        confirm_keyboard.add(InlineKeyboardButton('Edit', callback_data='edit'))

        bot.send_message(chat_id,
                         f"Please confirm your information:\nName: {first_name}\nSurname: {last_name}\nPhone: {phone_number}",
                         reply_markup=confirm_keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'confirm')
def confirm_information(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id) 
    chat_id = call.message.chat.id
    if chat_id in user_state:
        user_info = user_state[chat_id]
        name = user_info['name']
        surname = user_info['surname']
        phone_number = user_info['phone']

        try:
            new_user = models.User(
                created_at=datetime.utcnow(),
                updated_at=datetime.utcnow(),
                avatar_url="", 
                name=name,
                surname=surname,
                phone=phone_number,
                balance=0.0,
                is_admin=False,
                email_verified=False,
                telegram_id=chat_id
            )

            db.add(new_user)
            db.commit()
            db.refresh(new_user)

            bot.send_message(chat_id, f"Thank you! Your registration is completed.\n\n"
                                      f"Name: {name}\nSurname: {surname}\nPhone: {phone_number}")
            del user_state[chat_id]

        except IntegrityError as e:
            db.rollback()
            if 'unique' in str(e.orig).lower():
                bot.send_message(chat_id, "A user with this email or phone number already exists.")
            else:
                bot.send_message(chat_id, "An error occurred during registration. Please try again.")
        except Exception as e:
            db.rollback()
            print(f"Error during user registration: {e}")
            bot.send_message(chat_id, "An unexpected error occurred. Please try again later.")
        finally:
            db.close()

        bot.send_message(chat_id, "Registration completed successfully.", reply_markup=telebot.types.ReplyKeyboardRemove())


@bot.callback_query_handler(func=lambda call: call.data == 'edit')
def edit_information(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id)  
    chat_id = call.message.chat.id
    if chat_id in user_state:
        edit_keyboard = InlineKeyboardMarkup(row_width=2)
        edit_keyboard.add(
            InlineKeyboardButton('Edit First Name', callback_data='edit_name'),
            InlineKeyboardButton('Edit Surname', callback_data='edit_surname'),
            InlineKeyboardButton('Edit Phone Number', callback_data='edit_phone'),
            InlineKeyboardButton('Cancel', callback_data='cancel_edit')
        )

        bot.edit_message_text(chat_id=chat_id, message_id=call.message.message_id,
                              text="Which information would you like to edit?", reply_markup=edit_keyboard)

@bot.callback_query_handler(func=lambda call: call.data.startswith('edit_'))
def edit_field(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id)  
    chat_id = call.message.chat.id
    field = call.data.split('_')[1] 

    if chat_id in user_state:
        user_state[chat_id]['step'] = f'edit_{field}'

        if field == 'name':
            prompt = "Please enter your new first name:"
        elif field == 'surname':
            prompt = "Please enter your new surname:"
        elif field == 'phone':
            prompt = "Please enter your new phone number:"
        else:
            bot.send_message(chat_id, "Invalid option selected.")
            return

        bot.send_message(chat_id, prompt)

        bot.edit_message_reply_markup(chat_id, call.message.message_id, reply_markup=None)

@bot.message_handler(func=lambda message: message.chat.id in user_state and user_state[message.chat.id]['step'].startswith('edit_'))
def handle_edit_input(message: telebot.types.Message):
    chat_id = message.chat.id
    user_step = user_state[chat_id]['step']
    field = user_step.split('_')[1]

    new_value = message.text.strip()

    if field in ['name', 'surname', 'phone']:
        user_state[chat_id][field] = new_value

        user_state[chat_id]['step'] = 'confirm'

        confirm_keyboard = InlineKeyboardMarkup()
        confirm_keyboard.add(InlineKeyboardButton('Confirm', callback_data='confirm'))
        confirm_keyboard.add(InlineKeyboardButton('Edit', callback_data='edit'))

        bot.edit_message_text(chat_id=chat_id, message_id=message.message_id - 1,
                              text=f"Please confirm your updated information:\n"
                                   f"Name: {user_state[chat_id]['name']}\n"
                                   f"Surname: {user_state[chat_id]['surname']}\n"
                                   f"Phone: {user_state[chat_id]['phone']}",
                              reply_markup=confirm_keyboard)
    else:
        bot.send_message(chat_id, "An error occurred. Please try editing again.")

@bot.callback_query_handler(func=lambda call: call.data == 'cancel_edit')
def cancel_edit(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id)  
    chat_id = call.message.chat.id
    if chat_id in user_state:
        user_state[chat_id]['step'] = 'confirm'

        confirm_keyboard = InlineKeyboardMarkup()
        confirm_keyboard.add(InlineKeyboardButton('Confirm', callback_data='confirm'))
        confirm_keyboard.add(InlineKeyboardButton('Edit', callback_data='edit'))

        bot.edit_message_text(chat_id=chat_id, message_id=call.message.message_id,
                              text=f"Your information:\n"
                                   f"Name: {user_state[chat_id]['name']}\n"
                                   f"Surname: {user_state[chat_id]['surname']}\n"
                                   f"Phone: {user_state[chat_id]['phone']}",
                              reply_markup=confirm_keyboard)

@bot.callback_query_handler(func=lambda call: True)
def handle_unknown_callback(call: telebot.types.CallbackQuery):
    bot.answer_callback_query(call.id, text="Unknown action.")
    bot.send_message(call.message.chat.id, "Sorry, I didn't understand that action.")


bot.infinity_polling()

db.close()