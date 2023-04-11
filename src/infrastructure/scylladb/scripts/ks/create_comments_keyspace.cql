CREATE KEYSPACE IF NOT EXISTS comments With REPLICATION = { 'class': 'NetworkTopologyStrategy',
    'replication_factor' :3 }