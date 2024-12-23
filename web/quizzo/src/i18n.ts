import {
  I18nObject,
  SimpleI18n,
  getString,
  getArray,
  getRecord,
} from '@frontend/util/i18n'

const fallback: I18nObject = {
  company: 'Quizzo',
  bio: 'Bio',
  cancel: 'Cancel',
  edit: 'Edit',
  email: 'Email',
  link: 'Link',
  location: 'Location',
  logout: 'Log Out',
  name: 'Name',
  not_found: 'Not Found',
  not_found_text: "Sorry, we can't find what you're looking for.",
  password: 'Password',
  save: 'Save',
  send: 'Send',
  home: {
    hero: 'Quizzo',
    hero_text: 'A Quizzo for small to medium sized Vue apps.',
    details: 'Example Section',
    details_text:
      'Replace this text and add an image on the right, or remove the section entirely.',
  },
  auth: {
    login: 'Log In',
    sign_up: 'Sign Up',
    modal_text: 'You were signed out, please log in again.',
    sign_up_subtitle: 'Create an account.',
    no_account: "Don't have an account?",
    click_here: 'Click here to sign up!',
    password: 'Password',
    confirm_password: 'Confirm Password',
    have_account: 'Already have an account?',
    login_here: 'Log in here.',
    terms_agree: 'I agree to the ',
    forgot_password: 'Forgot Password?',
    forgot_password_subtitle:
      'Enter the email associated with your account to reset the password.',
    forgot_password_sent:
      'Thank you. A recovery email will be sent if the provided email matches an account.',
    back_to: 'Back to <>',
    update_password: 'Update Password',
    update_password_error: 'The code is invalid or expired. Want to try again?',
    update_password_text: 'Enter a new password to access your account.',
    confirm: 'Confirm Email',
    confirmed:
      'Your email is confirmed! Click to go to the Sites Dashboard, or close the tab.',
    confirm_expired: 'Code expired, click to resend.',
    confirm_expired_login:
      'Code expired, please log in and resend from the settings page.',
    confirm_resent:
      'Confirmation email resent to {email}, it should arrive in a minute or two!',
    confirm_again: 'Try again later or resend the code from the settings page.',
    resend: 'Resend',
  },
  profile: {
    title: 'Profile',
    settings: 'Settings',
    edit: 'Edit Profile',
    email_text: 'Your email is used for notifications and account recovery.',
    password_text: 'Set a unique password to secure your account.',
    update_email: 'Update Email',
    new_email: 'New email',
    update_password: 'Update Password',
    old_password: 'Old password',
    new_password: 'New password',
    confirm_password: 'Confirm new password',
    sign: 'Sign a message to confirm your address change',
    no_subscriptions: 'No Subscriptions',
    create: 'Create Product',
  },
  settings: {
    title: 'Settings',
  },
  footer: {
    copyright: '© 2024 Quizzo - All Rights Reserved.',
    title: 'Frontend Vue3 app template',
    text: 'Built with Vue, Vite, and Postcss. Get started instantly with a full featured template.',
    col1: 'Resources',
    col2: 'Quick Links',
    faq: 'FAQ',
    about: 'About',
    terms: 'Terms',
    privacy: 'Privacy Policy',
    cookie: 'Cookie Policy',
  },
  errors: {
    password_length: 'Password must be at least 8 characters.',
    name_len: 'Name must be between 2 and 20 characters.',
    confirm_password: 'Passwords do not match.',
    UserExists: 'Email is already in use.',
    InvalidFormData: 'Invalid input',
    InvalidAuth: 'Invalid email or password',
    None: 'Unknown error',
  },
}
export const i18n = new SimpleI18n(fallback)
export const ts = getString(i18n)
export const ta = getArray(i18n)
export const tr = getRecord(i18n)
