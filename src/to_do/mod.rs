use self::structs::{pending::Pending, done::Done};

pub mod structs;

pub enum ItemTypes{
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(type_: &str, title_: &str) ->
    Result<ItemTypes, &'static str> {

        if type_ == "pending" {
            let item = Pending::new(title_);
            Ok(ItemTypes::Pending(item))
        } else if type_ == "done" {
            let item = Done::new(title_);
            Ok(ItemTypes::Done(item))
        } else {
            Err("[!]This task not exist.")
        }

}