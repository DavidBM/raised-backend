This is one experiment I'm trying to understand better rust with threads and entity component systems better. It doesn't have any pretension and is only for fun.

As of now it creates a process that listen websockets (thread A) for new clients. For each new client it moves them to a waiting room (thread B) and when the room is full (4 players) it starts a game in a new game (thread C). This game executes each tic 40 times per second. For each tick it executes all the systems (each one a different thread) with an immutable world, generating effects (patches over the world). Once all are finished, the game thread applies these patches to the main world and then send the effects to the players via the websocket connection.

If you want to try it, just run the server with `cargo run` and then execute this code in your browser. 


```
(function hola () {
    let clients = [
        new WebSocket("ws://localhost:3012"),
        new WebSocket("ws://localhost:3012"),
        new WebSocket("ws://localhost:3012"),
        new WebSocket("ws://localhost:3012")
    ];
    setTimeout(() => {
        clients[0].send(JSON.stringify({"t":"move", "direction": 1.47}));
        clients[0].addEventListener("message", function(event) {console.log(JSON.parse(event.data))});
    }, 10);
    
})();
```

I thought in moving this to async but I don't feel that the benefit would pay off. On top of that, having several games in parallel may en with a game blocking the others and having a tic system may be difficult. Maybe next I try to implement an Actor system instead of a entity component system.
