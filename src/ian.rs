pub struct Interval {
	left: f64,
	right: f64,
}

impl<'a, 'b> std::ops::Add<&'b Interval> for &'a Interval {
	type Output = Interval;
	fn add(self, addend: &'b Interval) -> Interval {
		return Interval {
			left: self.left + addend.left,
			right: - (-self.right - addend.right)
		}
	}
}

impl<'a>  std::ops::Add<&'a Interval> for Interval {
	type Output = Interval;
	fn add(self, addend: &'a Interval) -> Interval {
		return Interval {
			left: self.left + addend.left,
			right: - (-self.right - addend.right)
		}
	}
}

impl<'a>  std::ops::Add<Interval> for &'a Interval {
	type Output = Interval;
	fn add(self, addend: Interval) -> Interval {
		return Interval {
			left: self.left + addend.left,
			right: - (-self.right - addend.right)
		}
	}
}

impl<'a, 'b>  std::ops::Sub<&'b Interval>  for &'a Interval {
	type Output = Interval;
	fn sub(self, subtrahend: &'b Interval) -> Interval {
		return Interval {
			left: self.left - subtrahend.right,
			right: self.right - subtrahend.left
		}
	}
}

impl<'a>  std::ops::Sub<&'a Interval>  for Interval {
	type Output = Interval;
	fn sub(self, subtrahend: &'a Interval) -> Interval {
		return Interval {
			left: self.left - subtrahend.right,
			right: self.right - subtrahend.left
		}
	}
}

impl<'a>  std::ops::Sub<Interval>  for &'a Interval {
	type Output = Interval;
	fn sub(self, subtrahend:Interval) -> Interval {
		return Interval {
			left: self.left - subtrahend.right,
			right: self.right - subtrahend.left
		}
	}
}

impl<'a, 'b>  std::ops::Mul<&'b Interval>  for &'a Interval {
	type Output = Interval;
	fn mul(self, multiplicand: &'b Interval) -> Interval {
		let tvec:Vec<f64> = vec!(
			self.left * multiplicand.left,
			self.left * multiplicand.right,
		    self.right * multiplicand.left,
		    self.right * multiplicand.right
		    );
		    
		return Interval {
			left: tvec.iter().cloned().fold(0./0., f64::min),
			right: tvec.iter().cloned().fold(0./0., f64::max)
		}
	}
}

impl<'a>  std::ops::Mul<&'a Interval>  for Interval {
	type Output = Interval;
	fn mul(self, multiplicand: &'a Interval) -> Interval {
		let tvec:Vec<f64> = vec!(
			self.left * multiplicand.left,
			self.left * multiplicand.right,
		    self.right * multiplicand.left,
		    self.right * multiplicand.right
		    );
		    
		return Interval {
			left: tvec.iter().cloned().fold(0./0., f64::min),
			right: tvec.iter().cloned().fold(0./0., f64::max)
		}
	}
}

impl<'a>  std::ops::Mul<Interval>  for &'a Interval {
	type Output = Interval;
	fn mul(self, multiplicand: Interval) -> Interval {
		let tvec:Vec<f64> = vec!(
			self.left * multiplicand.left,
			self.left * multiplicand.right,
		    self.right * multiplicand.left,
		    self.right * multiplicand.right
		    );
		    
		return Interval {
			left: tvec.iter().cloned().fold(0./0., f64::min),
			right: tvec.iter().cloned().fold(0./0., f64::max)
		}
	}
}

impl<'a, 'b>  std::ops::Div<&'b Interval>  for &'a Interval {
		type Output = Interval;
	fn div(self, divider: &'b Interval) -> Interval {
		if self.right == 0.0 || divider.right == 0.0 {
		  panic!("right point can't be 0.0 in division")
  	}
		let tvec:Vec<f64> = vec!(
			self.left / divider.left,
			self.left / divider.right,
		    self.right / divider.left,
		    self.right / divider.right
		    );
		return Interval {
			left: tvec.iter().cloned().fold(0./0., f64::min),
			right: tvec.iter().cloned().fold(0./0., f64::max)
		}
	}
}

impl<'a>  std::ops::Div<&'a Interval> for Interval {
		type Output = Interval;
	fn div(self, divider: &'a Interval) -> Interval {
		if self.right == 0.0 || divider.right == 0.0 {
		  panic!("right point can't be 0.0 in division")
  	}
		let tvec:Vec<f64> = vec!(
			self.left / divider.left,
			self.left / divider.right,
		    self.right / divider.left,
		    self.right / divider.right
		    );
		return Interval {
			left: tvec.iter().cloned().fold(0./0., f64::min),
			right: tvec.iter().cloned().fold(0./0., f64::max)
		}
	}
}

impl<'a>  std::ops::Div<Interval> for &'a Interval {
		type Output = Interval;
	fn div(self, divider: Interval) -> Interval {
		if self.right == 0.0 || divider.right == 0.0 {
		  panic!("right point can't be 0.0 in division")
  	}
		let tvec:Vec<f64> = vec!(
			self.left / divider.left,
			self.left / divider.right,
		    self.right / divider.left,
		    self.right / divider.right
		    );
		return Interval {
			left: tvec.iter().cloned().fold(0./0., f64::min),
			right: tvec.iter().cloned().fold(0./0., f64::max)
		}
	}
}

impl<'a> std::ops::AddAssign<&'a Interval>  for  Interval {
		type Output = Interval;
	fn add_assign(&mut self, addend: &'a Interval) {
		*self = Interval {
			left: self.left + addend.left,
			right: self.right + addend.right
		};
	}
}

impl<'a> std::ops::SubAssign<&'a Interval> for Interval {
		type Output = Interval;
	fn sub_assign(&mut self, subtrahend: &'a Interval) {
		*self = Interval {
		left: self.left - subtrahend.right,
		right: self.right - subtrahend.left
	    }
	}
}

impl<'a> std::ops::MulAssign<&'a Interval> for Interval {
		type Output = Interval;
	fn mul_assign(&mut self, multiplicand: &'a Interval) {
		let tvec:Vec<f64> = vec!(
			self.left * multiplicand.left,
			self.left * multiplicand.right,
		    self.right * multiplicand.left,
		    self.right * multiplicand.right
		    );
		*self = Interval {
		left: tvec.iter().cloned().fold(0./0., f64::min),
		right: tvec.iter().cloned().fold(0./0., f64::max)
	    }
	}
}

impl<'a> std::ops::DivAssign<&'a Interval> for Interval {
		type Output = Interval;
	fn div_assign(&mut self, divider: &'a Interval) {
		let tvec:Vec<f64> = vec!(
			self.left / divider.left,
			self.left / divider.right,
		    self.right / divider.left,
		    self.right / divider.right
		    );
		*self = Interval {
		left: tvec.iter().cloned().fold(0./0., f64::min),
		right: tvec.iter().cloned().fold(0./0., f64::max)
	    }
	}
}

impl<'a> std::ops::Add<f64> for &'a Interval {
		type Output = Interval;
	fn add(self, addend:f64) -> Interval {
		return Interval {
			left: self.left + addend,
			right: self.right + addend
		}
	}
}

impl<'a> std::ops::Sub<f64> for &'a Interval {
		type Output = Interval;
	fn sub(self, subtrahend:f64) -> Interval {
		return Interval {
			left: self.left - subtrahend,
			right: self.right - subtrahend
		}
	}
}

impl<'a> std::ops::Mul<f64> for &'a Interval {
		type Output = Interval;
	fn mul(self, multiplicand:f64) -> Interval {
		return Interval {
			left: self.left * multiplicand,
			right: self.right * multiplicand
		}
	}
}

impl<'a> std::ops::Div<f64> for &'a Interval {
		type Output = Interval;
	fn div(self, divider:f64) -> Interval {
		return Interval {
			left: self.left / divider,
			right: self.right / divider
		}
	}
}


impl<'a> std::ops::Add<&'a Interval> for f64 {
	type Output = Interval;
	fn add(self, addend: &'a Interval) -> Interval {
		return addend + self
	}
}

impl std::ops::Add<Interval> for f64 {
	type Output = Interval;
	fn add(self, mut addend: Interval) -> Interval {
		addend.left += self;
		addend.right += self;
		return addend;
	}
}

impl<'a> std::ops::Sub<&'a Interval> for f64 {
	type Output = Interval;
	fn sub(self, subtrahend: &'a Interval) -> Interval {
		return subtrahend - self
	}
}

impl<'a> std::ops::Mul<&'a Interval> for f64 {
	type Output = Interval;
	fn mul(self, multiplicand: &'a Interval) -> Interval {
		return multiplicand * self
	}
}

impl<'a> std::ops::Div<&'a Interval> for f64 {
	type Output = Interval;
	fn div(self, dividend: &'a Interval) -> Interval {
		return dividend / self
	}
}

impl<'a> std::ops::AddAssign<&'a f64> for  Interval {
		type Output = Interval;
	fn add_assign(&mut self, addend: &'a f64) {
		*self = Interval {
			left: self.left + addend,
			right: self.right + addend
		};
	}
}

impl<'a> std::ops::SubAssign<&'a f64> for  Interval {
		type Output = Interval;
	fn sub_assign(&mut self, subtrahend: &'a f64) {
		*self = Interval {
			left: self.left - subtrahend,
			right: self.right - subtrahend
		};
	}
}

impl<'a> std::ops::MulAssign<&'a f64> for  Interval {
		type Output = Interval;
	fn mul_assign(&mut self, multiplicand: &'a f64) {
		*self = Interval {
			left: self.left * multiplicand,
			right: self.right * multiplicand
		};
	}
}

impl<'a> std::ops::DivAssign<&'a f64> for  Interval {
		type Output = Interval;
	fn div_assign(&mut self, divider: &'a f64) {
		*self = Interval {
			left: self.left / divider,
			right: self.right / divider
		};
	}
}



impl std::fmt::Display for Interval {
	
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Interval LEFT[{:e}] RIGHT[{:e}]", self.left, self.right)
	}
	
	
}



impl Interval {
	
	pub fn new(lval: f64, rval: f64) -> Interval {
	return Interval { left: lval, right: rval }
    }
    
    pub fn from(ivl:&Interval) -> Interval {
	return Interval { left: ivl.left, right: ivl.right }
    }
    
    pub fn map(ivl:&Interval, f: fn(f64) -> f64) -> Interval {
		Interval {
			left: f(ivl.left),
		    right: f(ivl.right)
		}
	}
	
	
}
