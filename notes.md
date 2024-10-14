Architecture:

Have `results` struct whose key is the parameters and entry is the results. When seeking results, check hashmap for that result.
- need ordered keys
- flat hashmap or binary (or other) tree?
- in which case, serialised storage can include results
	+ but should have clear/refresh button
	+ could have 'get results' and 'get fresh results' buttons

parameters
- 2 teams
- (usually) each team has one soldier type (could do mixes!)
- soldier type details
- fight settings ((a)symmetric, # of fights)