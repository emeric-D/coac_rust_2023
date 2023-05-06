use std::io;

/*
DEPIN Emeric, 1A MKTRO
ENS Rennes
COAC 1A 2022-2023
*/ 

const MAP_SIZE:usize = 5; // Grid size

fn create_empty_map(mapsize:usize) -> Vec<i32> {
    // Function to create an empty grid
    let mut grille = Vec::new();
    for _i in 0..mapsize {
        for _j in 0..mapsize{
            grille.push(0);
        }
    }
    grille
}

fn add_boat(grille: &mut Vec<i32>, boat_list: &mut Vec<Vec<usize>>, i:usize, j:usize, taille:usize, direction:usize) -> (){
    // Function to add a boat on a grid, i j define the upper left position
    let mut boat = Vec::new();
    if direction == 0 {
        for k in 0..taille {
            change_case(grille, i, j+k, 1);
            boat.push(i*MAP_SIZE+j+k);
        }
    }
    else {
        for k in 0..taille {
            change_case(grille, i+k, j, 1);
            boat.push((i+k)*MAP_SIZE+j);
        }
    }
    boat_list.push(boat);
}

fn check_victory(grille: &mut Vec<i32>)->i32{
    // Function which check all positions of the grid, if there is a boat untouch, return 0, else return 1
    for i in 0..MAP_SIZE{
        for j in 0..MAP_SIZE{
            if grille[i*MAP_SIZE+j] == 1{
                return 0
            }
        }
    }
    return 1
}
fn change_case(grille: &mut Vec<i32>, i:usize, j:usize, elt: i32) -> (){
    // Function to change a case of the grid
    grille[i*MAP_SIZE+j] = elt;
}

/*
fn print_grille(grille: &mut Vec<i32>) -> (){
    // Function to print the grid (one grid)
    for i in 0..MAP_SIZE {
        for j in 0..MAP_SIZE{
            print!("{}",grille[MAP_SIZE * i + j]);
        }
        println!();
    }
}
*/

fn print_grids(grille_j: &mut Vec<i32>, grille_a: &mut Vec<i32>) ->(){
    // Function to print the grid of the player and the grid of the adversary
    println!("Play   Opn");
    for i in 0..MAP_SIZE {
        for j in 0..MAP_SIZE{
            print!("{}",grille_j[MAP_SIZE * i + j]);
        }
        print!("  ");
        for j in 0..MAP_SIZE{
            if grille_a[MAP_SIZE * i + j]==1{
                print!("0");
            }else{
                print!("{}",grille_a[MAP_SIZE * i + j]);
            }
        }
        println!();
    }
}

fn touch_boat(grille: &mut Vec<i32>, position:usize, boat_list: &mut Vec<Vec<usize>>) -> (){
    // Function to change the state of a boat
    let begin:usize = 0;
    grille[position] = 2;
    for i in begin..boat_list.len(){
        for j in begin..(boat_list[i]).len(){
            if boat_list[i][j] == position{
                for pos in boat_list[i].iter(){
                    if grille[*pos] != 2{
                        return;// If a case of the boat is not touched, the boat is not sunk
                    }
                    // If all the cases of the boat are touched, the boat is sunk
                }
                println!("Je dirais même plus, bateau coulé !!!");
                for pos in boat_list[i].iter(){
                    // Change the state of all case  of the boat to sunk
                    grille[*pos] = 3;
                }
            }
        }
    }
}

fn input(to_print: String)->usize{
    // Function to get an input from the player and return it as a usize
    println!("{}",to_print);
    let mut input_value = String::new(); 
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");
    let input_value: usize = input_value.trim().parse().expect("Entrée invalide");
    return input_value;
}

fn player_play(player_grid: &mut Vec<i32>, adv_grid: &mut Vec<i32>, adv_boat_list: &mut Vec<Vec<usize>>, player: &mut i32)->i32{
    let position: usize;
    println!("C'est au tour du joueur {} de jouer !", *player + 1);
    print_grids(player_grid, adv_grid);
    let colonne = input("Colonne :".to_string());
    let ligne = input("Ligne :".to_string());
    position = ligne*MAP_SIZE+colonne;
    if adv_grid[position] == 1{
        println!("Bateau touché !");
        touch_boat(adv_grid, position, adv_boat_list);
        if check_victory(adv_grid) == 1 {
            println!("Joueur {} a gagné !", *player + 1);
            return 1;
        }
    }else if adv_grid[position] == 0{
        println!("Coup dans l'eau !");
        adv_grid[position] = 4;
        *player = 1 - *player;
    }else{
        *player = 1 - *player;
    }
    return 0;
}

fn computer_play_rd(player_grid: &mut Vec<i32>, boat_list_player: &mut Vec<Vec<usize>>, player: &mut i32)->i32{
    // Function to play for the computer
    let mut position:usize;
    println!("Au tour de l'ordinateur de jouer !");
    loop{
        position = rand::random::<usize>()%(MAP_SIZE*MAP_SIZE);
        if player_grid[position] == 1{
            println!("Bateau touché !");
            touch_boat(player_grid, position, boat_list_player);
            if check_victory(player_grid) == 1 {
                println!("L'ordinateur a gagné !");
                return 1
            }
        }else if player_grid[position] == 0{
            println!("Coup dans l'eau !");
            player_grid[position] = 4;
            *player = 1 - *player;
            break;
        }
    }
    return 0;
    //println!("L'ordinateur a joué en {} {}", position/MAP_SIZE, position%MAP_SIZE);
}

/*
fn computer_algo(player_grid: &mut Vec<i32>, boat_list_player: &mut<Vec<Vec<usize>>, player: &mut i32)->i32{
    let mut position:usize;
}
*/

fn game()->(){
    // Function to play the game
    // Initialisation of the variables
    let mut grille_j1 = create_empty_map(MAP_SIZE);
    let mut grille_j2 = create_empty_map(MAP_SIZE);
    let mut boat_list_j1 = Vec::new();
    let mut boat_list_j2 = Vec::new();
    let mut player = 0;
    let mut victory:i32;
    // Initialisation of the boats
    for boat in [[1, 1, 2, 1] , [3, 1, 2, 0]].iter(){
        add_boat(&mut grille_j1, &mut boat_list_j1, boat[0], boat[1], boat[2], boat[3]);
    }
    for boat in [[1, 1, 2, 0] , [2, 1, 2, 1]].iter(){
        add_boat(&mut grille_j2, &mut boat_list_j2, boat[0], boat[1], boat[2], boat[3]);
    }
    // Game loop
    loop{
        if player == 0{
            victory = player_play(&mut grille_j1, &mut grille_j2, &mut boat_list_j2, &mut player);
        }else{
            victory = computer_play_rd(&mut grille_j1, &mut boat_list_j1, &mut player);
            //player_play(&mut grille_j2, &mut grille_j1, &mut boat_list_j1, &mut player);
        }
        if victory == 1{
            break;
        }
    }
    
}


//Pour la grille, 0 correspond a rien, 1 a un bateau, 2 a un bateau touche, 3 a un bateau coule, 4 un coup dans l'eau
// Deux jours différents, player 0 et player 1
fn main() {
    game();
}
