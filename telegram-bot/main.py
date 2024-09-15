from datetime import datetime
import os
from dotenv import load_dotenv
import telebot
from telebot.types import InlineKeyboardMarkup, InlineKeyboardButton, ReplyKeyboardMarkup, KeyboardButton
from sqlalchemy.orm import Session
from database import SessionLocal, engine
import models
from sqlalchemy.exc import IntegrityError


load_dotenv()

BOT_TOKEN = os.getenv("BOT_TOKEN")

if not BOT_TOKEN:
    raise ValueError("BOT_TOKEN is not set in the environment variables.")

bot = telebot.TeleBot(BOT_TOKEN)

user_state = {}  

@bot.message_handler(commands=['start', 'help'])
def send_welcome(message):
    keyboard = InlineKeyboardMarkup()
    keyboard.add(InlineKeyboardButton('Register', callback_data='register'))
    keyboard.add(InlineKeyboardButton('Button 2', callback_data='button2'))

    bot.send_message(message.chat.id, 'Please choose an option:', reply_markup=keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'register')
def handle_register(call):
    bot.answer_callback_query(call.id) 
    keyboard = InlineKeyboardMarkup()
    keyboard.add(InlineKeyboardButton('New User', callback_data='new_user'))
    keyboard.add(InlineKeyboardButton('Connect Account', callback_data='connect_account'))

    bot.edit_message_text(chat_id=call.message.chat.id, message_id=call.message.message_id,
                          text="You pressed the Register button!", reply_markup=keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'new_user')
def start_registration(call):
    bot.answer_callback_query(call.id)  
    chat_id = call.message.chat.id
    user_state[chat_id] = {'step': 'name'} 
    contact_keyboard = ReplyKeyboardMarkup(one_time_keyboard=True, resize_keyboard=True)
    contact_keyboard.add(KeyboardButton('Share Contact', request_contact=True))

    bot.send_message(chat_id, "Please share your contact information:", reply_markup=contact_keyboard)

@bot.message_handler(content_types=['contact'])
def handle_contact(message):
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
def confirm_information(call):
    bot.answer_callback_query(call.id) 
    chat_id = call.message.chat.id
    if chat_id in user_state:
        user_info = user_state[chat_id]
        name = user_info['name']
        surname = user_info['surname']
        phone_number = user_info['phone']

        db: Session = SessionLocal()
        try:
            import bcrypt
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
def edit_information(call):
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
def edit_field(call):
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
def handle_edit_input(message):
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
def cancel_edit(call):
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
def handle_unknown_callback(call):
    bot.answer_callback_query(call.id, text="Unknown action.")
    bot.send_message(call.message.chat.id, "Sorry, I didn't understand that action.")


bot.infinity_polling()