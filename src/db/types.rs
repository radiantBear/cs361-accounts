use diesel::result::Error as DieselError;


pub enum Error {
    DieselError(DieselError),
    CustomError(String)
}