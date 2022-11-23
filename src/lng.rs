use farmanager_codegen::Langpack;
use farmanager::basic;

#[derive(Langpack, Copy, Clone)]
#[langpack(name = "clipselrs")]
#[language(code = "en", value = "English,English")]
#[language(code = "ru", value = "Russian,Russian (Русский)")]
pub enum Lng {

    #[msg(language = "en", value = "Select From Clipboard")]
    #[msg(language = "ru", value = "Select From Clipboard")]
    MenuItemTitle,

    #[msg(language = "en", value = "Select from Clipboard!")]
    #[msg(language = "ru", value = "Select from Clipboard!")]
    MessageTitle,

    #[msg(language = "en", value = "")]
    #[msg(language = "ru", value = "")]
    MessageLine0,

    #[msg(language = "en", value = "Files selected: ")]
    #[msg(language = "ru", value = "Выделено файлов: ")]
    MessageLine1,

    #[msg(language = "en", value = "&Ok")]
    #[msg(language = "ru", value = "Угу")]
    MessageButton,

    #[msg(language = "en", value = "Error")]
    #[msg(language = "ru", value = "Ошибка")]
    ErrorTitle
}

