# Clustering

Clustering with good coffee

I change the build of ClusterOfSequence by adding a name :
It takes the first letter of all sequence contained and here your title
this process add 0.5ms to the clusterize process of hemoglobina (53.49s to 54.00s)
A way to build it differently :

- create a custom name for each cluster in the `create_foam_rep()` method

This way this feature `name` would only slow the method using it

create_foam_rep() :
<<<<<<< HEAD
First approch :

- give a name to cluster directly into their structure when being created
- When creating a cluster with only one sequence give the full sequence name to the cluster
- When creating a cluster with a list of sequence, give the concat of all first letter of those sequence
- When merging two cluster, give the concat of the two clusters' name

Second approch :
=======
First approach :
 - give a name to cluster directly into their structure when being created
 - When creating a cluster with only one sequence give the full sequence name to the cluster
 - When creating a cluster with a list of sequence, give the concat of all first letter of those sequence
 - When merging two cluster, give the concat of the two clusters' name

Second approach :
 - Cluster with one sequence : named by the sequence
 - Cluster with more : named with the three first letters of each sequence (separated by `-`)
 - Cluster of Cluster : named with sub_clusters' name separated with `_`
>>>>>>> bbc93a2aa5df848efa0126ae6c0684188988c636

- Cluster with one sequence : named by the sequence
- Cluster with more : named with the three first letters of each sequence (separated by `-`)
- Cluster of Cluster : named with sub_clusters' name separated with `_`

---
