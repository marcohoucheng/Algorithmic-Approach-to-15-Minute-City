import pandas as pd, networkx as nx, heapq, time, sys

def custom_dijkstra(G, source, threshold, i):

    global reach, new_node_id

    # Initialize distances and visited set
    distances = {node: float('inf') for node in G.nodes}
    distances[source] = 0
    # visited = set()

    # Priority queue (heap) to store nodes and their distances
    pq = [(0, source)]

    while pq:
        current_distance, current_node = heapq.heappop(pq)

        # Stop if all nodes within 15 have been visited
        if current_distance > threshold:
            break
        
        if current_node != new_node_id:
            reach[current_node][i] = 1

        # if current_node not in visited:
            # visited.add(current_node)

        for neighbor, edge_data in G[current_node].items():
            weight = min([dict(edge_data)[k]['weight'] for k in dict(edge_data).keys()])
            new_distance = current_distance + weight
            if new_distance < distances[neighbor]:
                distances[neighbor] = new_distance
                heapq.heappush(pq, (new_distance, neighbor))

    return

if __name__ == '__main__':

    if len(sys.argv) == 2:
        t = int(sys.argv[1])
        repeat = 1
    elif len(sys.argv) == 3:
        t = int(sys.argv[1])
        repeat = int(sys.argv[2])
    else:
        print('Usage: python main.py <threshold> <repeat>')
        sys.exit(1)

    edges_df = pd.read_csv('./min_city/data/edges.csv')
    nodes_df = pd.read_csv('./min_city/data/nodes.csv')

    graph_df = nx.from_pandas_edgelist(edges_df, 'source', 'target', ['weight'], create_using=nx.MultiGraph())

    # Assuming your nodes_df has columns 'node', 'name', and 'gender'
    nx.set_node_attributes(graph_df, pd.Series(nodes_df['label'], index=nodes_df['id']).to_dict(), 'label')

    for node_id in graph_df.nodes():
        graph_df.nodes[node_id]['label'] = nodes_df.loc[node_id, 'label']

    services = set()
    for d in list(dict(graph_df.nodes(data=True)).values()):
        services.add(d['label'])
    services = {x for x in services if x==x}

    service_nodes_df = {k: list(v) for k, v in nodes_df.groupby('label')['id']}

    total_duration = 0
    
    threshold = 0 if repeat == 1 else (10 if repeat // 10 > 10 else repeat // 10)

    reach_org = {key: [0] * len(services) for key in graph_df.nodes()}

    for i in range(repeat):

        reach = reach_org.copy()
        
        start = time.time()

        for idx, service_type in enumerate(services):
            new_node_id = max(graph_df.nodes) + idx + 1
            graph_df.add_node(new_node_id) # create new node
            graph_df.add_weighted_edges_from([(new_node_id, node, 0) for node in service_nodes_df[service_type]])
            custom_dijkstra(graph_df, new_node_id, t, idx)
            graph_df.remove_node(new_node_id)
            # FMC_df = FMC_df.intersection(FMC_tmp)

        FMC_df = {k for k, v in reach.items() if sum(v) == len(services)}
        
        end = time.time()

        if i >= threshold:
            total_duration += end - start

    if repeat > 1:
        print(f"Average execution time over {repeat - threshold} iterations for {t}-MC: {total_duration / (repeat - threshold)}")
    else:
        print(f"Execution time for {t}-MC: {total_duration}")

    print(f'There are {len(FMC_df)} nodes in the {t} MC')