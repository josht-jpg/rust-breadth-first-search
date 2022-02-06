struct Queue<T> {
    pub items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push(v)
    }

    pub fn dequeue(&mut self) -> T {
        self.items.remove(0)
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

type Vertex = Vec<u32>;
type Graph = Vec<Vertex>;

fn bfs(graph: Graph, start_node: u32, end_node: u32) -> Vec<Option<u32>> {
    let mut queue = Queue::new();
    queue.enqueue(start_node);

    let mut visisted_vertices = vec![false; graph.len()];
    visisted_vertices[0] = true;

    let mut prev: Vec<Option<u32>> = vec![None; graph.len()];

    'outer: while !queue.is_empty() {
        let current_node = queue.dequeue();
        for v in graph[current_node as usize].iter() {
            if *v == end_node {
                prev[*v as usize] = Some(current_node);
                break 'outer;
            }

            if !visisted_vertices[*v as usize] {
                queue.enqueue(*v);
                visisted_vertices[*v as usize] = true;
                prev[*v as usize] = Some(current_node);
            }
        }
    }

    let mut path = Vec::new();
    let mut at = Some(end_node);
    while at != None {
        path.push(at);
        at = prev[at.unwrap_or(0) as usize];
    }

    path.reverse();

    return match path[0] {
        Some(x) if x == start_node => path,
        _ => vec![],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_path() {
        let G: Graph = vec![
            vec![1, 2],
            vec![0, 3, 4, 1],
            vec![0, 4],
            vec![1, 4, 5],
            vec![1, 2, 3, 5],
            vec![3, 4, 6],
            vec![5, 7],
            vec![6],
        ];

        assert_eq!(
            bfs(G, 0, 7),
            vec![Some(0), Some(1), Some(3), Some(5), Some(6), Some(7)]
        )
    }

    #[test]
    fn no_existing_path() {
        let G: Graph = vec![
            vec![1, 2, 5],
            vec![0, 1, 3, 4],
            vec![0, 3],
            vec![1, 4, 5, 2],
            vec![1, 3, 5],
            vec![0, 3, 4, 1],
            vec![7],
            vec![6],
        ];

        assert_eq!(bfs(G, 0, 7), vec![])
    }
}
