# wasm-flownet-api

This flow function exposes a HTTPS POST endpoint for simple log in. 

It accepts two parameters: ```username``` and ```password```

The parameters must be sent as JSON in a HTTPS POST request body.

The api endpoint is: ```https://code.flows.network/lambda/3xcOkrII9F```


The flow function checks the password against 4 conditions to determine whether a login attempt
is successful or not.

The four conditions are:
```
 "Is at least 8 characters long",
 "Starts with an uppercase letter",
 "Contains both an alphabet character and a number",
 "Contains a special character"
```
A score is used to determine login success. The score initializes at 0, 
every password condition met increases the score by 25%. A 100% score guarantees successful log in.

## Usage

You can use the `curl` command to access the flow function.

```
 curl -X POST -H "Content-Type: application/json" -d '{"username":"<your username>", "password":"<your password>"}' https://code.flows.network/lambda/3xcOkrII9F

```
e.g

```
 curl -X POST -H "Content-Type: application/json" -d '{"username":"testuser", "password":"password"}' https://code.flows.network/lambda/3xcOkrII9F

```
You can also access the flow function using a script. In Javascript for instance, you can do:

```
const data = { username: "testuser", password: "password" };

fetch('https://code.flows.network/lambda/3xcOkrII9F', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
})
.then(response => response.text())
.then(text => console.log(text))
.catch(error => console.error('An error occurred:', error));

```
You may also use any other HTTPS client such as Postman or any other that can send a request body to the POST endpoint of this function.
