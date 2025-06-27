# Rust TowerDefense-Prototype-1

![Capture d'écran 2025-06-27 145605](https://github.com/user-attachments/assets/3555ca56-202c-4f20-8c92-97212f9405b1)

## Installation

The prototype is written in [Rust](https://www.rust-lang.org). If you do not have a rust toolchain on your system 
yet, [install](https://www.rust-lang.org/tools/install) it, rustup is the preferred way to change that. When the toolchain is ready, go in your the file, build and run the
game with

    cargo run --release
    

## How to play

When you start the protoype, you spawn directly on the game: <br>

![Capture d'écran 2025-06-27 145829](https://github.com/user-attachments/assets/7a872ff7-9cca-4c98-b957-e7ab632b58e1)

- On the screen there is nothing at the beginning expect the map and the golds written en top-right. <br>

Click on 1, 2 or 3 at the top of the keyboard to choose to place a tower. <br>
There are 3 types of tower with different statistics (mentioned in the same order as the keycodes): <br>
- Archer (Key 1): 1 attack damage, 5 range, 1.5 attack speed and attack single target. <br>
- Boomer (Key 2): 3 attack damage, 3 range, 0.7 attack speed and do same damages in a radius of 3. <br>
- Knight (Key 3): 5 attack damage, 2 range, 1.0 attack speed and do damages to all target in his range. <br>

When you press 1, 2 or 3 on your keyboard, you are preparing placing a tower, 1 for Archer, 2 for Boomer and 3 for Knight. There is also an indication of which tower you are placing on top left by the letter A (Archer), B (Boomer) and K (Knight). <br>
To place the tower just click on the screen with your mouse after have pressed a mentionned keycode. Also you need 10 gold to place any towers. <br>
You can also stop placing tower by pressing escape button. <br>

Enemies spawn every second have 10 base health and a speed of 100. Every 15 enemies a big enemy spawn, it have 50 base health and a speed of 75. You can consider that every 15 enemies it start another wave cause every waves the health of enemies grows. <br>
Yes, enemies or tanks. x) <br>

The game is infinite, you have no health, just enemies spawning and having health higher and higher. <br>

## Note

This is my first prototype of a Tower Defense game or a game with dynamics animations and my first prototype using bevy. That's why it is not much and it's not a real game, I didn't spend much times on balancing and upgrades. I didn't wanted to make a difficult and long prototype. Just a classic infinite tower defense where you just place towers and don't even upgrade them. By doing that project, I wanted to manipulate bevy and train with Rust. <br>
I'm new in game development world but I really love that and I want to continue so if someone read that and want to help me I'll take every advice in game development, rust development, game design and even how to make commentaries. <br>
By the way if I do some errors or you think something isn't clear or anything in this or in commentaries in my games don't hesitate to tell me ! <br>
You can fin me on: <br>
- discord: papplushh <br>
- instagram: @papplush <br>
- x (previously twitter): @Papplush <br>
