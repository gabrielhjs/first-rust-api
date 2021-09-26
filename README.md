# Rust API

My first functional rust api.

---

<center>


[Features](#Features) |
[Routes](#Routes) |
[TODOs](#TODOs) |
[Me](https://github.com/gabrielhjs)


</center>

---

### Features

* [Rust](https://www.rust-lang.org)
   * [Actix](https://actix.rs/)
    * [Serde](https://serde.rs/)

---

### Routes

<table>
<tr>
<th>Endpoint</th>
<th>Method</th>
<th>Ex. req</th>
<th>Ex. resp</th>
<th>Function</th>
</tr>

<tr>
<td>/hello</td>
<td><i>POST</i></td>
<td>

```json
{
  "data": "hey"
}
```

</td>
<td>

```json
{
  "data": "Hey, you said: hey"
}
```

</td>
<td><i>
The response says what you entered in the date field.
</i></td>

</tr>

<tr>
<td>/status</td>
<td><i>GET</i></td>
<td></td>
<td>

```json
{
  "status": "Ok"
}
```

</td>
<td><i>
The response shows the current status of the api.
</i></td>

</tr>
</table>

---

### TODOs

* [x] Status;
* [x] Some post data with request body;
* [x] Apply Docker;
* [x] Apply Docker Compose;
* [x] Some connection with database using [Diesel](https://diesel.rs/);
* [x] Insert data in some table;
* [ ] Read data from table;
* [ ] Cache with [Redis](https://redis.io/);
* [ ] Web [Socket](https://actix.rs/docs/websockets/) connection;
* [ ] Streaming database and send to socket;
* [x] Create user;
* [ ] Auth users;
* [ ] Logout users;
* [ ] Some logged route access.

---

by [Gabriel Sá](https://github.com/gabrielhjs) | Software Developer
