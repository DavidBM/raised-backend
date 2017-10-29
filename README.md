# Raised backend

This is a small eperiments that I started for learning the the entitiy-component-system and websockets in rust plus for training some threaded logic.

Basically it does this:

1) Creates an event loop for the websocket library.
2) Creates a queue that execute a new game for every 4 connected players

The game refresh the worls several times every second. The world in an inmutable structure and the components create patches for every update that are applies at the end of the update cicle. From every update a user update message is generated and send to the correct users via websocket.
