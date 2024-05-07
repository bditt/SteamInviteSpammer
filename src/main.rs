use steamworks::*;
use std::{thread, time};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{0}", args.len());
    if args.len() != 3
    {
        println!("Invalid arguments: ./SteamInviteSpammer.exe <gameid (Ex: 730 = CS2)> <Friend SteamID 64>");
        return;
    }

    let game_id = args[1].parse::<u32>().unwrap();;
    let steam_id = args[2].parse::<u64>().unwrap();;
    println!("Inviting {0} to {1}!", steam_id, game_id);
    let ten_millis = time::Duration::from_millis(100);
    let (client,single) = Client::init_app(AppId(game_id)).unwrap();
    let friends = client.friends();
    let friend_list = friends.get_friends(FriendFlags::ALL);
    println!("Friend List Size: {0}", friend_list.len());
    //let friend = friends.get_friend();
    for friend in friend_list
    {
        if friend.id().raw() == steam_id
        {
            for counter in 1..1000
            {
                friend.invite_user_to_game("Get fucking trolled loser!");
                thread::sleep(ten_millis);
            }
        }
    }
}
