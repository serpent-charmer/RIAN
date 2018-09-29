mod ian;


fn f( ivl: &ian::Interval) -> ian::Interval {
	return 5.0 + 6.0 * ivl - &ian::Interval::map(
	&ian::Interval::from(&(ivl + 1.0)), f64::exp
	);
}

fn main() {
	
	let args: Vec<String> = std::env::args().collect();
	println!("{:?}", args);
	let n:f64 = args[1].parse::<f64>().unwrap();
	let mut a:f64 = args[2].parse::<f64>().unwrap();
	let b:f64 = args[3].parse::<f64>().unwrap();
    let r:f64 = args[4].parse::<f64>().unwrap();
	
	let h:f64 = (b - a) / (n - 1.0);
	
	println!("\tN = {}\n\ta={}\n\tb={}\n\tr={}\n\th={}\n", n, a, b, r, h);
	
	
	
	while a <= b {
		println!("a = {}", a);
	 if a > 0.0
	   { 
		  let ivl:ian::Interval = ian::Interval::new(a - r, a + r);
		  println!("f[{}] = {}", ivl, f(&ivl));
	   }
	  else
	   { 
		   let ivl:ian::Interval = ian::Interval::new(a + r, a - r);
		   println!("f[{}] = {}", ivl, f(&ivl)); 
	   }
	 
	 
	 a += h;
	}

	
	
	
}
