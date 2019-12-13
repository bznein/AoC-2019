use itertools::Itertools;
use std::collections::HashSet;
use num::integer::lcm;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct P
{
 x: i64,
 y: i64,
 z: i64,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct V
{
 x: i64,
 y: i64,
 z: i64,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Moon
{
	p: P,
	v: V,
}

impl Moon{

	pub fn new(p : P) -> Moon
	{
		Moon {
			p : p,
			v : V { x: 0, y:0, z:0 },
		}
	}

}

fn total_energy(moons: &Vec<Moon>) -> i64
{
	let mut total = 0;
	for m in moons 
	{
		let potential = (m.p.x.abs()+m.p.y.abs()+m.p.z.abs());
		let kinetic = (m.v.x.abs()+m.v.y.abs()+m.v.z.abs());
		total += potential * kinetic;
	}
	total
}

fn is_equal_dir(moon1 : &Vec<Moon>, moon2 : &Vec<Moon>, dir: usize) -> bool
{
	let mut result = true;
	for i in 0..moon1.len()
	{
			match dir
			{
				0=> result = result && moon1[i].p.x == moon2[i].p.x && moon1[i].v.x == moon2[i].v.x,
				1=> result = result && moon1[i].p.y == moon2[i].p.y && moon1[i].v.y == moon2[i].v.y,
				2=> result = result && moon1[i].p.z == moon2[i].p.z && moon1[i].v.z == moon2[i].v.z,
				_ => panic!("Wrong direction"),
			}
	}
	result
}

fn universe_period_direction(mut moons: & mut Vec<Moon>, dir : usize) -> u64
{
	let initial_step = moons.clone();
	let mut steps: u64 = 0;
	loop
	{
		step_moons(& mut moons, dir);
		steps += 1;
		if is_equal_dir(&initial_step,moons,dir)
		{
			return steps;
		}
	}
}

fn universe_period(moons: &Vec<Moon>) -> u64
{
	let steps = lcm(universe_period_direction(& mut moons.clone(),0),universe_period_direction(&mut moons.clone(),1));
	lcm(universe_period_direction(&mut moons.clone(),2),steps)
}

fn step_moons(moons: & mut Vec<Moon>, dir: usize)
{
	for i in 0..4
		{
			for j in i+1..4
			{	
				match dir 
				{
					0 => {
						if (moons[i].p.x<moons[j].p.x)
						{
							moons[i].v.x+=1;
							moons[j].v.x-=1;
						}
						else if (moons[i].p.x>moons[j].p.x)
						{
							moons[i].v.x-=1;
							moons[j].v.x+=1;
						}
					}
					1 =>
					{
						if (moons[i].p.y<moons[j].p.y)
						{
							moons[i].v.y+=1;
							moons[j].v.y-=1;
						}
						else if (moons[i].p.y>moons[j].p.y)
						{
							moons[i].v.y-=1;
							moons[j].v.y+=1;
						}
					}
					2 =>
					{
						if (moons[i].p.z<moons[j].p.z)
						{
							moons[i].v.z+=1;
							moons[j].v.z-=1;
						}
						else if (moons[i].p.z>moons[j].p.z)
						{
							moons[i].v.z-=1;
							moons[j].v.z+=1;
						}
					}
					_ => panic!("Wrong direction"),
				}
			}
		}
		for i in 0..4
		{
			match dir
			{
				0=> moons[i].p.x+=moons[i].v.x,
				1=> moons[i].p.y+=moons[i].v.y,
				2=> moons[i].p.z+=moons[i].v.z,
				_ => panic!("Wrong direction"),
			}
		}	
}

/* Note: I hardcode input here as I honestly
do not care about parsing it, it's just not worth it */
fn main() {
	let mut moons = Vec::new();
	moons.push(Moon::new(P{ x: -6, y:-5, z: -8}));
	moons.push(Moon::new(P{ x: 0, y:-3, z: -13}));
	moons.push(Moon::new(P{ x: -15, y:10, z: -11}));
	moons.push(Moon::new(P{ x: -3, y:-8, z: 3}));
	
	let period = universe_period(&moons.clone());
	
	for _i in 0..1000
	{
		step_moons(& mut moons,0);
		step_moons(& mut moons,1);
		step_moons(& mut moons,2);
	}
	println!("Part 1  {}", total_energy(&moons));
	println!("Part 2: {}", period);
}
