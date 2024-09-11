import telebot
from telebot.types import InlineKeyboardMarkup, InlineKeyboardButton, ReplyKeyboardMarkup, KeyboardButton

bot = telebot.TeleBot("6785855039:AAH1zQz61x1zSevCF8sVUlWMyc3fdYZPrIU")

user_state = {}  # Dictionary to store user states and data


@bot.message_handler(commands=['start', 'help'])
def send_welcome(message):
    keyboard = InlineKeyboardMarkup()
    keyboard.add(InlineKeyboardButton('Register', callback_data='register'))
    keyboard.add(InlineKeyboardButton('Button 2', callback_data='button2'))

    bot.send_message(message.chat.id, 'Please choose an option:', reply_markup=keyboard)



@bot.callback_query_handler(func=lambda call: call.data == 'register')
def handle_register(call):
    keyboard = InlineKeyboardMarkup()
    keyboard.add(InlineKeyboardButton('New User', callback_data='new_user'))
    keyboard.add(InlineKeyboardButton('Connect Account', callback_data='connect_account'))

    # Edit the current message instead of sending a new one
    bot.edit_message_text(chat_id=call.message.chat.id, message_id=call.message.message_id,
                          text="You pressed the Register button!", reply_markup=keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'new_user')
def start_registration(call):
    chat_id = call.message.chat.id
    user_state[chat_id] = {'step': 'name'}  # Initialize state
    contact_keyboard = ReplyKeyboardMarkup(one_time_keyboard=True, resize_keyboard=True)
    contact_keyboard.add(KeyboardButton('Share Contact', request_contact=True))

    bot.send_message(chat_id, "Please share your contact information:", reply_markup=contact_keyboard)
    #
    # bot.edit_message_text(chat_id=chat_id, message_id=call.message.message_id,
    #                       text="Please enter your name:")


@bot.message_handler(content_types=['contact'])
def handle_contact(message):
    chat_id = message.chat.id

    if message.contact:
        first_name = message.contact.first_name
        last_name = message.contact.last_name if message.contact.last_name else ""
        phone_number = message.contact.phone_number

        # Store the contact information and prompt for confirmation
        user_state[chat_id] = {'name': first_name, 'surname': last_name, 'phone': phone_number, 'step': 'confirm'}

        # Create confirmation keyboard
        confirm_keyboard = InlineKeyboardMarkup()
        confirm_keyboard.add(InlineKeyboardButton('Confirm', callback_data='confirm'))
        confirm_keyboard.add(InlineKeyboardButton('Edit', callback_data='edit'))

        bot.send_message(chat_id,
                         f"Please confirm your information:\nName: {first_name}\nSurname: {last_name}\nPhone: {phone_number}",
                         reply_markup=confirm_keyboard)

@bot.callback_query_handler(func=lambda call: call.data == 'confirm')
def confirm_information(call):
    chat_id = call.message.chat.id
    if chat_id in user_state:
        user_info = user_state[chat_id]
        name = user_info['name']
        surname = user_info['surname']
        phone_number = user_info['phone']

        bot.send_message(chat_id, f"Thank you! Your registration is completed.")
        del user_state[chat_id]  # Clear the state after confirmation

@bot.callback_query_handler(func=lambda call: call.data == 'edit')
def edit_information(call):
    chat_id = call.message.chat.id
    if chat_id in user_state:
        # Reset the state to allow the user to share contact information again
        user_state[chat_id]['step'] = 'name'
        contact_keyboard = ReplyKeyboardMarkup(one_time_keyboard=True, resize_keyboard=True)
        contact_keyboard.add(KeyboardButton(text=user_state[chat_id]['name'])

        bot.send_message(chat_id, "Please share your contact information again:", reply_markup=contact_keyboard)

bot.infinity_polling()