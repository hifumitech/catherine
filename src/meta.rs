/*
    Project: Catherine Framework (https://github.com/battleoverflow/catherine)
    Author: battleoverflow (https://github.com/battleoverflow)
    License: BSD 2-Clause
*/

use crate::catherine::VERSION;

// Catherine banners
pub fn banners() -> Vec<String> {

    let banners: Vec<String> = vec![
        format!(r"
         _____       _   _               _            
        /  __ \     | | | |             (_)           
        | /  \/ __ _| |_| |__   ___ _ __ _ _ __   ___ 
        | |    / _` | __| '_ \ / _ \ '__| | '_ \ / _ \
        | \__/\ (_| | |_| | | |  __/ |  | | | | |  __/
         \____/\__,_|\__|_| |_|\___|_|  |_|_| |_|\___|

                                              v{}
        ", VERSION),

        format!(r"
          |\                     /)
        /\_\\__               (_//
       |   `>\-`     _._       //`)
        \ /` \\  _.-`:::`-._  //
         `    \|`    :::    `|/
               |     :::     |
               |.....:::.....|
               |:::::::::::::|
               |     :::     |
               \     :::     /
                \    :::    /
                 `-. ::: .-'
                  //`:::`\\
                 //   '   \\
                |/         \\
                      
              Catherine | v{}
        ", VERSION),

        format!(r"
          ⣿⣿⣿⣿⣿⣿⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠁⠀⣿
          ⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠙⠿⠿⠿⠻⠿⠿⠟⠿⠛⠉⠀⠀⠀⠀⠀⣸⣿
          ⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣴⣿⣿⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⢰⣹⡆⠀⠀⠀⠀⠀⠀⣭⣷⠀⠀⠀⠸⣿⣿⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠈⠉⠀⠀⠤⠄⠀⠀⠀⠉⠁⠀⠀⠀⠀⢿⣿⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⢾⣿⣷⠀⠀⠀⠀⡠⠤⢄⠀⠀⠀⠠⣿⣿⣷⠀⢸⣿⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⡀⠉⠀⠀⠀⠀⠀⢄⠀⢀⠀⠀⠀⠀⠉⠉⠁⠀⠀⣿⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿
          ⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿
          Catherine | v{}
        ", VERSION),

        format!(r"
          |\__/,|   (`\
          |_ _  |.--.) )
          ( T   )     /
        (((^_(((/(((_/
        Catherine | v{}
        ", VERSION),
    ];

    return banners;
}
