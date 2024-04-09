use std::io;

struct UserData{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

fn main() {
    let mut counter: u64 = 0;
    let mut last_user: UserData = UserData{
        active:false,
        username: String::new(),
        email: String::new(),
        sign_in_count: 0
    };
    loop{
        counter += 1;
        println!("Enter name:");
        let mut user_name = String::new();
        io::stdin()
            .read_line(&mut user_name)
            .expect("Failed to read input!");
        println!("Enter email:");
        let mut user_email = String::new();
        io::stdin()
            .read_line(&mut user_email)
            .expect("Failed to read input!");
        let user_data = get_user_data(user_name, user_email, counter, last_user);
        println!("\nactive:{} \nname:{} \nemail:{} \ncounter:{}\n", user_data.active, user_data.username.trim(), user_data.email.trim(), user_data.sign_in_count);
        last_user = user_data;
        println!("Exit? (y = yes)");
        let mut exit_res = String::new();
        io::stdin()
            .read_line(&mut exit_res)
            .expect("Failed to read input!");
        if exit_res.trim() == "y"{
            println!("Have an awesome day!");
            break;
        }
    }
}

fn get_user_data(name: String, email: String, sign_in: u64, last_user:UserData) -> UserData{
    let activity: bool = last_user.active;
    if activity {
        println!("\nDeactivating last user first!\n");
        UserData{
            active: !&activity,
            sign_in_count: sign_in,
            ..last_user
        }
    }else{
        UserData{
            active: !&activity,
            username: name,
            email: email,
            sign_in_count: sign_in
        }
    }
}