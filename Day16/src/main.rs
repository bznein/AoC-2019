use std::io::{self, Read};
use std::iter;
use std::char;

fn repeat_pattern(v: &[i32], count: usize) -> Vec<i32> {
	v.iter().flat_map(|&x| {
        iter::repeat(x).take(count)
    }).collect::<Vec<i32>>()
}

fn fft(v: & mut Vec<i32>, phases: usize) -> Vec<i32>
{
	let pattern = vec![1,-1];
	let mut zeros = vec![0;v.len()];
	let mut vs = vec![v,& mut zeros ];
	for p in 0..phases
	{
		let new_v_index = 1-(p%2);
		let v_index = p%2;		
		for i in 0..vs[0].len()
		{
			vs[new_v_index][i] = 0;
			let n = vs[v_index].len();
			let mut j=i;
			let mut steps = 0;
			for ii in pattern.iter().flat_map(|&x| {
				iter::repeat(x).take(i+1)
				}).cycle().take(n/2)
			{
				let jval = vs[v_index][j];
				vs[new_v_index][i] += jval*ii;
				if steps == i 
				{
					j+=i+2;
					steps = 0;
				}
				else
				{
					j+=1;
					steps+=1;
				}
				if j >= n
				{
					break;
				}
			}
			vs[new_v_index][i] = (vs[new_v_index][i]%10).abs();
		}
	}
	vs[phases%2 as usize].to_vec()
}

fn fft_offset(v: & mut Vec<i32>, phases: usize, offset: usize)-> Vec<i32> 
{
	let mut zeros = vec![0;v.len()];
	let mut vs = vec![v,& mut zeros ];
	let len = vs[0].len();
	for p in 0..phases
	{
		let new_v_index = 1-(p%2);
		let v_index = p%2;	
		for i in (offset..len).rev()
		{	
			if i == len-1
			{
				vs[new_v_index][i] = vs[v_index][i];
			}
			else 
			{
				vs[new_v_index][i] = (vs[v_index][i]+vs[new_v_index][i+1]);
			}
		}
		for i in (offset..len).rev()
		{
			vs[new_v_index][i] = vs[new_v_index][i]%10;
		}
	}
	vs[phases%2 as usize].to_vec()
	
}

fn main() { 
	let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let mut v: Vec<i32> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
	
	let v_clone = v.clone();
    let mut p2_v: Vec<i32> = v_clone.iter().cycle().take(v.len() * 10_000).map(|x| *x).collect();	
	let message_offset = 5979673;
	let phases = 100;
	v = fft(&mut v, phases);
	println!("{:?}", &v[0..8]);
	p2_v = fft_offset(& mut p2_v, phases,message_offset);
	println!("{:?}", &p2_v[message_offset..message_offset+8]);
	

}
