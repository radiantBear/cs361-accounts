import assert from 'node:assert'

const API_KEY = process.env.API_KEY;
const BASE_URL = 'http://localhost:3000'
console.log();


/************************************ Create a user *************************************/
let res = await fetch(BASE_URL+'/users', { 
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': API_KEY
    },
    body: JSON.stringify({ 
        username: 'test', 
        password: 'password' 
    })
});

assert.equal(res.status, 200);

let body = await res.json();
const user_id = body.id;
console.log(
    'User account created. You are responsible for making sure their chosen\n'
    + 'password is strong before making this request. You can perform account \n'
    + `initialization in other microservices using ${body.id} as the user ID\n`
);



/********************************** Log in as the user **********************************/
res = await fetch(BASE_URL+'/sessions', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': API_KEY
    },
    body: JSON.stringify({
        username: 'test', 
        password: 'password' 
    })
});

assert.equal(res.status, 200);

body = await res.json();
console.log(
    `Can store ${body.uuid}\n`
    + 'on the client (e.g. in a cookie or localStorage) to track the session\n'
);
const session_id = body.uuid;



/************************* Get the user's ID from their session *************************/
res = await fetch(BASE_URL+`/sessions/${session_id}`, {
    method: 'GET',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': API_KEY
    }
});

assert.equal(res.status, 200);

body = await res.json();
console.log(
    `Can reference the logged-in user in other microservices with ${body.id}\n`
    + 'as the user ID\n'
);



/********************************** Get a secure nonce **********************************/
res = await fetch(BASE_URL+'/nonces', {
    method: 'POST',
    headers: {
        'X-API-Key': API_KEY
    }
});

assert.equal(res.status, 200);

body = await res.json();
const nonce = body.nonce;
console.log(
    `Should embed the nonce ${body.nonce}\n`
    + 'to the page in such a way that it will be submitted with the sensitive request \n'
    + 'it protects (e.g. as a hidden form field)\n'
);



/********************************** Validate the nonce **********************************/
res = await fetch(BASE_URL+`/nonces/${nonce}`, {
    method: 'DELETE',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': API_KEY
    }
});

assert.equal(res.status, 200);

console.log(
    'Nonce was valid; this passed security check indicates the request has not\n'
    + 'been replayed\n'
);



/********************************* Validate used nonce *********************************/
res = await fetch(BASE_URL+`/nonces/${nonce}`, {
    method: 'DELETE',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': API_KEY
    }
});

assert.equal(res.status, 404);

console.log(
    'Nonce was no longer valid; this failed security check indicates the request\n'
    + 'may have been replayed\n'
);



/****************************** Delete the user's account *******************************/
res = await fetch(BASE_URL+`/users/${user_id}`, {
    method: 'DELETE',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': API_KEY
    }
});

assert.equal(res.status, 200);

console.log(
    'User account has been deleted. You are responsible for performing security\n'
    + 'checks and validating that the user is properly authorized to delete the account\n'
    + 'before making this request\n'
);
