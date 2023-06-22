# Breg organisasjonsroller
tanken med denne er å lage en liten app som henter ned org info fra breg sine åpne apier, og organisere det for oppslag via api. 
og å lære litt rust. 

Første commit fungerer ikke, og er utarbeidet i hovedsak av phind søkemotoren drevet av ChatGPT. 
dette er søket som ga koden:
https://www.phind.com/search?cache=6577fb84-4e7a-4c21-ab63-0e04d6fa8f92
```
i want to write a small rust program that exposes an http endpoint, and integrates with mongodb.
heres a description of what the program should do:
- expose an http endpoint that takes a path parameter on "/org/[9-digit number]", and return the json object identified by that 9-digit number in the db. 
- if the object is not in the db, search this api:
"https://data.brreg.no/enhetsregisteret/api/enheter/[9-digit number]/roller" for the information, and insert it into the db, and also return it to the http endpoint the program exposes. 
- the db credentials are user: "user" and password: "pass", the db url is "mongodb.local" 

plese request additional information if it is needed to complete the program. 
```


