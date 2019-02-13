import rdflib
import sys

g = rdflib.Graph()
g.parse("blog.rdf")
qres = g.query(sys.argv[1])
print(qres.serialize(format="json"))
