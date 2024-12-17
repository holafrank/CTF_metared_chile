
use std::{collections::VecDeque, io};

pub fn ctf_program() {

    /// "+" : Increases the value of the stack's last element by 1.
    fn plus_sign(a:&mut VecDeque<u8>) {
        if let Some(last) = a.back_mut() {
            *last += 1;
        }
    }

    /// "-" : Decreases the value of the stack's last element by 1.
    fn minus_sign(a:&mut VecDeque<u8>) {
        if let Some(last) = a.back_mut() {
            *last -= 1;
        }
    }

    ///"." : Duplicates the last element of the stack and adds it to the end.
    fn dot_sign(a:&mut VecDeque<u8>) {
        if let Some(&last) = a.back() {
            let aux = last.clone();
            a.push_back(aux);
        }
    }

    ///">" : Moves the first element to the end of the stack, shifting all others down.
    fn greater_than_sign(a:&mut VecDeque<u8>) {
        if let Some(first) = a.pop_front() {
            a.push_back(first);
        }
    }

    ///"<" : Moves the last element to the beginning of the stack, shifting all others up.
    fn less_than_sign(a:&mut VecDeque<u8>) {
        if let Some(last) = a.pop_back() {
            a.push_front(last);
        }
    }

    ///"@" : Swaps the last two elements of the stack.
    fn at_sign(a:&mut VecDeque<u8>) {
        if a.len() >= 2 {
            let last = a.pop_back().unwrap();
            let second_last = a.pop_back().unwrap();
            a.push_back(last);
            a.push_back(second_last);
        }
    }

    ///"€" : Outputs the ASCII representation of each element in the stack.
    fn output(a: &VecDeque<u8>) {
        print!("Flag: ");
        for &value in a {
            print!("{}", value as char);
        }
    }

    let mut array: VecDeque<u8> = VecDeque::new();
    array.push_front(0);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la línea");

    for operador in input.trim().chars() {

        if operador == '€' {
            output(&array);
            break;
        }

        match operador {
            '+' => plus_sign(&mut array),
            '-' => minus_sign(&mut array),
            '<' => less_than_sign(&mut array),
            '>' => greater_than_sign(&mut array),
            '.' => dot_sign(&mut array),
            '@' => at_sign(&mut array),
            _ => print!("Operador no válido!")
        }

    }

}