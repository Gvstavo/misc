use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Graph{

	pub v: i64,
	pub e: HashMap<i64 , LinkedList<i64>>

}

impl Graph{

	pub fn new(n: i64) -> Self{

		let mut hash = HashMap::with_capacity(n as usize);

		for i in 0..n{
			hash.insert(i,LinkedList::new());
		}

		Graph{
			v: n,
			e: hash
		}
	}

	pub fn add_edge(&mut self, v: i64 ,w: i64){
		let  a = self.e.get_mut(&v).unwrap();
		a.push_back(w);

	}

	pub fn  dfs(&self , v: i64){

		let mut map : HashMap<i64 , bool>  = HashMap::with_capacity(self.v as usize);

		for i in 0..self.v{
			
			map.insert(i,false);			
		}

		self.do_dfs(v,&mut map)
	}

	pub fn do_dfs(&self, v: i64,map: &mut HashMap<i64,bool>){

		let  vertice = map.get_mut(&v).unwrap();

		*vertice = true;

		let list = self.e.get(&v).unwrap();

		print!("{:?}",v );
		for element in list.iter() {
    	if !map.get(element).unwrap(){
    		self.do_dfs(*element, map)
    	}
		}
	}

	pub fn bfs(&self , v: i64){

		let mut map : HashMap<i64 , bool>  = HashMap::with_capacity(self.v as usize);

		for i in 0..self.v{
			
			if i != v{			
				map.insert(i,false);
			}
			else{
				map.insert(i,true);
			}			
		}
		let mut queue : VecDeque<i64>= VecDeque::new();	

		queue.push_back(v);

		while!queue.is_empty(){

			let s = queue.clone().front().unwrap().to_owned();
			print!("{:?}",s );

			queue.pop_front();
			let list = self.e.get(&s).unwrap();

			for i in list.iter(){

				if !map.get(i).unwrap(){
					let a = map.get_mut(i).unwrap();
					*a = true;
					queue.push_back(*i);
				}
			}
		}
	}
}