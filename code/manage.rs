﻿use std::cmp::Ordering;

fn main()
{

// Большая часть конструкций кода является выражением. В том числе, циклы, условия и т.п.
let _x = 
{
	println!("Начало");
	0	// последнее значение блока/функции является его возвращаемым значением
};


// -------------------------------- loop, while, if --------------------------------

	let mut i = 0;
	loop
	{
		println!("Число {}", i);
		// Тип условного выражения в if - всегда bool без конвертации целых чисел в bool
		if i >= 3
		{
			break;
		}
		else
		if false
		{
			continue;
		}
		else
		{
			// Если здесь поставить continue, то Rust определить, что код ниже недостижим
			// continue;
		}

		// if умеет возвращать значение.
		// Требуется аккуратность:
		// обе ветки кода должны возвращать значение одного и того же типа
		// т.к. это статически типизированный язык

		i +=
		if i == 0
		{
			2	// Нет точки с запятой!
		}
		else
		{
			1	// Нет точки с запятой!
		};		// Точка с запятой!
	}
	/*
		Вывод:
		Число 0
		Число 2
		Число 3
	*/


	// Нижележащий цикл вернёт одно значение на операторе break (кортеж из индекса элемента и его значения)
	let a = [1, 2, 3, 4, 5];

	let mut index = 0;
	let element5 = loop
	{
		if a[index] == 4
		{
			break (index, a[index]);
		}

		index += 1;
	};

	// a[3] == 4
	println!("a[{}] == {}", element5.0, element5.1);

	index = 0;
	while index < a.len()
	{
		index += 1;
	}

	// do .. while отсутствует
	// В for тоже можно делать break
	for _e in a.iter()
	{
		println!("a.iter(): {}", _e);
	}
	
	for (i, &e) in a.iter().enumerate()
	{
		println!("a.iter(): a[{}] == {}", i, e);
	}

/* Вывод двух предыдущих циклов
a.iter(): 1
a.iter(): 2
a.iter(): 3
a.iter(): 4
a.iter(): 5
a.iter(): a[0] == 1
a.iter(): a[1] == 2
a.iter(): a[2] == 3
a.iter(): a[3] == 4
a.iter(): a[4] == 5
*/

	for number in (1..4).rev()
	{
		println!("Цикл по number == {}", number);
		// Выведет 3 2 1
	}
	println!();

// -------------------------------- match --------------------------------

	let str1 = String::from("aaa");
	let str2 = String::from("0065536");

	let mut num1:u64 = 0;

	// let str1:u64 = match возвращает результат! (см. далее)
	// В таком случае нужно поставить ; после match, а уточнение типа в parse уже не нужно - будет само выведено
	match str1.parse::<u64>()
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        std::result::Result::Ok (num)  => num1 = num,
        std::result::Result::Err(err) => println!("Не число! Сообщение об ошибке:\r\n{}", err),
    }

	let num2:u64 = match str2.parse()
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ok (num) => num,
        Err(_)   => 	// Можно ввести _err вместо _ ; Здесь мы декларируем, что нам не нужно это значение
		{
			println!("Не число! Введённая строка: {}", str2);
			0
		},
    };

	let etalon:u64 = 2;
	match etalon.cmp(&num1)
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ordering::Less    => println!("{} < {}",  2, num1),
        Ordering::Greater => println!("{} > {}",  2, num1),
        Ordering::Equal   => println!("{} == {}", 2, num1),
    }

	match etalon.cmp(&num2)
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ordering::Less => println!("{} < {}", 2, num2),
		_ => println!("{} >= {}", 2, num2),
    }
}
