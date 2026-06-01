import "./Roompage.css"
import "./HomePage.css"
import { useLocation } from "react-router-dom";
import type { Room } from "./RoomBox";
import type { User } from "./UserCard";
import { useEffect, useState } from "react";
import UserCard from "./UserCard";

interface QueryUserResponse {
  users: User[];
}

export default function RoomPage() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const location = useLocation();
  const room = location.state?.room as Room | undefined;

  // ✅ Always call useEffect — put the condition inside
  useEffect(() => {
    const rToken = localStorage.getItem("roomToken");
    if (rToken || !room) return; // already have a token, skip joining

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
        localStorage.setItem("roomToken", data.message);
        if (!response.ok) {
          throw new Error("Failed to join room");
        }
      } catch (err) {
        console.error(err);
      }
    };

    joinRoom();
  }, [room]);

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
          throw new Error("Failed to fetch rooms");
        }
        const data: QueryUserResponse = await response.json();
        console.log(data);
        setUsers(data.users);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchUsers();
  }, []);

  if (loading) return <h1>Loading...</h1>;

  return (
    <main className="home">
      <div className="grid-overlay">
        <div style={{ position: "relative", zIndex: 1}}>
          <h1 className="room-title">WELCOME TO THE {room?.name.toUpperCase()} ROOM</h1>
        </div>
        <div className="users-grid">
          {users.map((user, index) => (
            <UserCard key={user.id} user={user} position={index + 1} />
          ))}
        </div>
      </div>
    </main>
  );
}
