use itertools::Itertools;
use std::collections::HashSet;
use num::integer::lcm;


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Moon
{
	p: Vec<i64>,
	v: Vec<i64>,
}

impl Moon{

	pub fn new(p: Vec<i64>) -> Moon
	{
		Moon {
			p : p,
			v : vec![0,0,0],
		}
	}

    pub fn is_equal_dir(&self, m : &Moon, dir: usize) -> bool
    {
        self.p[dir] == m.p[dir] && self.v[dir] == m.v[dir]
    }

}

fn total_energy(moons: &Vec<Moon>) -> i64
{
	let mut total = 0;
	for m in moons 
	{
		let potential = (m.p[0].abs()+m.p[1].abs()+m.p[2].abs());
		let kinetic = (m.v[0].abs()+m.v[1].abs()+m.v[2].abs());
		total += potential * kinetic;
	}
	total
}

fn is_equal_dir(moon1 : &Vec<Moon>, moon2 : &Vec<Moon>, dir: usize) -> bool
{
	let mut result = true;
	for i in 0..moon1.len()
	  {
        result = result && moon1[i].is_equal_dir(&moon2[i],dir);
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
						if (moons[i].p[dir]<moons[j].p[dir])
						{
							moons[i].v[dir]+=1;
							moons[j].v[dir]-=1;
						}
						else if (moons[i].p[dir]>moons[j].p[dir])
						{
							moons[i].v[dir]-=1;
							moons[j].v[dir]+=1;
						}		
			}
		}
		for i in 0..4
		{
        moons[i].p[dir]+=moons[i].v[dir];
		}	
}

/* Note: I hardcode input here as I honestly
do not care about parsing it, it's just not worth it */
fn main() {
	let mut moons = Vec::new();
	moons.push(Moon::new(vec![-6,-5,-8]));
	moons.push(Moon::new(vec![0,-3,-13]));
	moons.push(Moon::new(vec![-15, 10,-11]));
	moons.push(Moon::new(vec![-3,-8,3]));
	
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
