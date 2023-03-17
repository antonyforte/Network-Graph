use std::fs::File;
use std::io::Write;
use std::ops::Neg;
use petgraph::graph::{NodeIndex, Graph, Node};
use petgraph::dot::Dot;
use petgraph::prelude::DiGraph;
use petgraph::visit::IntoEdgesDirected;
use petgraph::visit::EdgeRef;


fn main() {

    //Criação do Grafo Direcionado (Graph - mapa da romenia(Não tem triangulos para teste do coeficiente de clustering))
    let mut graph = DiGraph::new();
    let mut graph2:Graph<&str,i32> = DiGraph::new();

    let a = graph2.add_node("a");
    let b = graph2.add_node("b");
    let c = graph2.add_node("c");
    let d = graph2.add_node("d");
    let e = graph2.add_node("e");
    let f = graph2.add_node("f");
    let g = graph2.add_node("g");
    let h = graph2.add_node("h");


    let b1 = graph2.add_edge(a,b,1);
    let b2 = graph2.add_edge(a,c,1);
    let b3 = graph2.add_edge(a,d,1);
    let b4 = graph2.add_edge(a,e,1);
    let b5 = graph2.add_edge(e,d,1);
    let b6 = graph2.add_edge(e,a,1);
    let b7 = graph2.add_edge(b,f,1);
    let b8 = graph2.add_edge(c,h,1);
    let b9 = graph2.add_edge(h,d,1);
    let b10 = graph2.add_edge(f,g,1);
    let b11 = graph2.add_edge(g,b,1);

    //Criação dos Nós do Grafo(Representando o mapa da romênia)
    let neamt = graph.add_node("Neamt");
    let iasi = graph.add_node("Iasi");
    let vaslui = graph.add_node("Vaslui");
    let urziceni = graph.add_node("Urziceni");
    let hirsova = graph.add_node("Hirsova");
    let eforie = graph.add_node("Eforie");
    let bucharest = graph.add_node("Bucharest");
    let giurgiu = graph.add_node("Giurgiu");
    let pitesti = graph.add_node("Pitesti");
    let craiova = graph.add_node("Craiova");
    let drobeta = graph.add_node("Drobeta");
    let mehadia = graph.add_node("Mehadia");
    let lugoj = graph.add_node("Lugoj");
    let timisoara = graph.add_node("Timisoara");
    let arad = graph.add_node("Arad");
    let zerind = graph.add_node("Zerind");
    let oradea = graph.add_node("Oradea");
    let remnicu_vilcea = graph.add_node("Remnicu_Vilcea");
    let sibiu = graph.add_node("Sibiu");
    let fagaras = graph.add_node("Fagaras");

    //Criação das Arestas do grafo
    let e1 = graph.add_edge(neamt,iasi,87);
    let e2 = graph.add_edge(iasi,vaslui,92);
    let e3 = graph.add_edge(vaslui,urziceni,142);
    let e4 = graph.add_edge(urziceni,hirsova,98);
    let e5 = graph.add_edge(hirsova,eforie,86);
    let e6 = graph.add_edge(urziceni,bucharest,85);
    let e7 = graph.add_edge(bucharest,urziceni,87);
    let e8 = graph.add_edge(bucharest,giurgiu,90);
    let e9 = graph.add_edge(bucharest,fagaras,211);
    let e10 = graph.add_edge(fagaras,sibiu,99);
    let e11 = graph.add_edge(sibiu,oradea,151);
    let e12 = graph.add_edge(oradea,zerind,71);
    let e13 = graph.add_edge(zerind,arad,75);
    let e14 = graph.add_edge(arad,sibiu,140);
    let e15 = graph.add_edge(arad,timisoara,118);
    let e16 = graph.add_edge(timisoara,lugoj,111);
    let e17 = graph.add_edge(lugoj,mehadia,70);
    let e18 = graph.add_edge(mehadia,drobeta,75);
    let e19 = graph.add_edge(drobeta,craiova,120);
    let e20 = graph.add_edge(craiova,pitesti,138);
    let e21 = graph.add_edge(pitesti,bucharest,101);
    let e22 = graph.add_edge(sibiu,remnicu_vilcea,80);
    let e23 = graph.add_edge(remnicu_vilcea,pitesti,97);

    //Quantidade de nós do grafo
    let len = graph.node_count();
    let len2 = graph2.node_count();

    
    //exemplos de utilização da função (Só substitua os números)
    //println!("{}",Dot::new(&graph));
    //println!("{}",node_degree(&graph, 18));
    //println!("{}",clustering_coefficient(&graph2,a));
    //println!("{}",degree_distribution(&graph, 1, len));
    
    // FUNÇÃO DO TP DE REDES (SUBSTITUA GRAPH E LEN PARA INFO DE OUTROS GRAFOS)
    complex_networks_tp1(&graph, len);

    //Traduz o grafo para o formato DOT
    let dot = Dot::new(&graph2);
    
    //Cria um arquivo .DOT Para depois gerar uma imagem PNG
    let mut file = File::create("/home/byakko/Documentos/Faculdade/Redes Complexas/Network-Graph/output/graph2.dot").expect("Can't create file");
    write!(file,"{}",dot).expect("Failed do write in file");
}

//Função que calcula o grau de um Determinado Nó 
fn node_degree(graph: &Graph<&str,i32>, n_index: usize) -> usize {
    let nodin = NodeIndex::new(n_index);
    let nodesin = graph.edges_directed(nodin,petgraph::Incoming).count();
    let nodesout = graph.edges_directed(nodin, petgraph::Outgoing).count();
    return nodesin+nodesout;
}

//Função que calcula o coeficiente de clusterização

fn clustering_coefficient(graph: &DiGraph<&str,i32>, node: NodeIndex) -> f32 {
    let mut neighbors = graph.neighbors_directed(node, petgraph::Direction::Outgoing);
    let mut count = 0;
    let mut triangles = 0;

    while let Some(n1) = neighbors.next() {
        for n2 in neighbors.clone() {
            if graph.find_edge(n1, n2).is_some() {
                triangles += 1;
            }
        }
        count += 1;
    }

    if count < 2 {
        return 0.0;
    }

    let possible_triangles = (count * (count - 1)) / 2;
    triangles as f32 / possible_triangles as f32
}

//Calcula a Distriubuição de graus
fn degree_distribution(graph: &Graph<&str,i32>,degree: usize,len: usize) -> f64 {
    let mut count: usize = 0;
    let mut node_count:usize = 0;
    while count <= len {
        if node_degree(&graph,count) >= degree{
            node_count = node_count + 1;
        }
        count = count + 1;
    }
    let result = node_count as f64 / len as f64;
    return result;
} 

fn complex_networks_tp1(graph: &Graph<&str,i32>,len: usize){
    let mut count = 0;

    println!("Grau de Todos os vértices");
    while count <= len {
        let mut weight = graph.node_weight(NodeIndex::new(count));
        println!("{:?} - {}",weight,node_degree(&graph, count));
        count = count + 1;
    }
    count = 0;
    println!("");

    println!("Coeficiente De Clustering");
    while count <= len {
        let mut weight = graph.node_weight(NodeIndex::new(count));
        println!("{:?} - {}",weight,clustering_coefficient(&graph,NodeIndex::new(count)));
        count = count + 1;
    }

    count = 1;
    println!("");

    println!("Distribuição de Graus");
    while count <= len {
        if degree_distribution(&graph, count, len) == 0.0{
            break;
        }
        println!("{} - {}",count, degree_distribution(&graph, count, len));
        count = count + 1;
    }


}
