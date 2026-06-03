import "./Roompage.css";
import "./HomePage.css";

import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import type { Room } from "./RoomBox";
import type { User } from "./UserCard";

import UserCard from "./UserCard";

interface QueryUserResponse {
  users: User[];
}

interface QueryIdResponse {
  id: number;
}

interface SocketMessage {
  type: string;
  payload?: any;
}

export default function RoomPage() {
  const [users, setUsers] = useState<User[]>([]);
  const [currentUserId, setCurrentUserId] = useState<number | null>(null);

  const [usersLoaded, setUsersLoaded] = useState(false);
  const [idLoaded, setIdLoaded] = useState(false);

  const location = useLocation();
  const navigate = useNavigate();

  const room = location.state?.room as Room | undefined;

  // =========================
  // JOIN ROOM
  // =========================

  useEffect(() => {
    const rToken = localStorage.getItem("roomToken");

    if (rToken || !room) return;

    const joinRoom = async () => {
      try {
        const token = localStorage.getItem("authToken");

        const response = await fetch("/api/joinroom", {
          method: "POST",
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            room_id: room.id,
            pop: true,
            password: "",
          }),
        });

        const data = await response.json();

        if (!response.ok) {
          throw new Error("Failed to join room");
        }

        localStorage.setItem("roomToken", data.message);
      } catch (err) {
        console.error(err);
      }
    };

    joinRoom();
  }, [room]);

  // =========================
  // WEBSOCKET
  // =========================

  useEffect(() => {
    const roomToken = localStorage.getItem("roomToken");

    if (!roomToken) return;

    const protocol =
      window.location.protocol === "https:"
        ? "wss:"
        : "ws:";

    const ws = new WebSocket(
      `${protocol}//${window.location.host}/ws?room_token=${roomToken}`
    );

    ws.onopen = () => {
      console.log("WebSocket connected");
    };

    ws.onmessage = (event) => {
      try {
        const message: SocketMessage = JSON.parse(event.data);

        switch (message.type) {
          case "GAME_STARTED":
            console.log("Game started");

            if (!room) {
              console.error("Room missing");
              return;
            }

            navigate(`/app/game/${room.id}`, {
              state: { room },
            });

            break;
          case "USER_JOINED":
            console.log("A player joined");
            break;

          default:
            console.log("Unknown message:", message);
        }
      } catch (err) {
        console.error(err);
      }
    };

    ws.onclose = () => {
      console.log("WebSocket disconnected");
    };

    return () => {
      ws.close();
    };
  }, [navigate, room]);

  // =========================
  // USERS IN ROOM
  // =========================

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        const token = localStorage.getItem("roomToken");

        if (!token) {
          throw new Error("Missing room token");
        }

        const response = await fetch("/api/query/usersinroom", {
          method: "GET",
          headers: {
            Authorization: `Bearer ${token}`,
            "Room-Token": token,
          },
        });

        if (!response.ok) {
          throw new Error("Failed to fetch users");
        }

        const data: QueryUserResponse = await response.json();

        setUsers(data.users);
      } catch (err) {
        console.error(err);
      } finally {
        setUsersLoaded(true);
      }
    };

    fetchUsers();
  }, []);

  // =========================
  // CURRENT USER ID
  // =========================

  useEffect(() => {
    const fetchId = async () => {
      try {
        const token = localStorage.getItem("authToken");

        if (!token) {
          throw new Error("Missing auth token");
        }

        const response = await fetch("/api/query/idbyauthtoken", {
          method: "GET",
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!response.ok) {
          throw new Error("Failed to fetch user ID");
        }

        const data: QueryIdResponse = await response.json();

        setCurrentUserId(data.id);
      } catch (err) {
        console.error(err);
      } finally {
        setIdLoaded(true);
      }
    };

    fetchId();
  }, []);

  // =========================
  // START GAME
  // =========================

  const handleStartGame = async () => {
    console.log("BUTTON CLICKED");
    try {
      const token = localStorage.getItem("authToken");
      const roomToken = localStorage.getItem("roomToken");

      if (!token || !roomToken) {
        throw new Error("Missing token");
      }

      const response = await fetch("/api/startgame", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
          "Room-Token": roomToken,
        },
      });

      if (!response.ok) {
        throw new Error("Failed to start game");
      }

      console.log("Start game request sent");
    } catch (err) {
      console.error(err);
    }
  };

  // =========================
  // STATE
  // =========================

  const loading = !usersLoaded || !idLoaded;

  const isAdmin =
    currentUserId !== null &&
    users.some(
      (user) =>
        String(user.id) === String(currentUserId) &&
        !user.typeof_user
    );

  if (loading) {
    return <h1>Loading...</h1>;
  }

  // =========================
  // ADMIN VIEW
  // =========================

  if (isAdmin) {
    return (
      <main className="home">
        <div className="grid-overlay">
          <div style={{ position: "relative", zIndex: 1 }}>
            <h1 className="room-title">
              YOU ARE AN ADMIN FOR THIS ROOM
            </h1>
          </div>

          <div className="hero-buttons">
            <button
              className="primary-btn"
              onClick={handleStartGame}
            >
              Start Game
            </button>
          </div>

          <div className="users-grid">
            {users.map((user, index) => (
              <UserCard
                key={user.id}
                user={user}
                position={index + 1}
              />
            ))}
          </div>
        </div>
      </main>
    );
  }

  // =========================
  // PLAYER VIEW
  // =========================

  return (
    <main className="home">
      <div className="grid-overlay">
        <div style={{ position: "relative", zIndex: 1 }}>
          <h1 className="room-title">
            WELCOME TO THE {room?.name.toUpperCase()} ROOM
          </h1>
        </div>
        <div className="users-grid">
          {users.map((user, index) => (
            <UserCard
              key={user.id}
              user={user}
              position={index + 1}
            />
          ))}
        </div>
      </div>
    </main>
  );
}
