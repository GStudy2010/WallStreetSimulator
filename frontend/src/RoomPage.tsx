import GoBack from "./GoBack"
import "./Roompage.css"
import "./HomePage.css"
import { useLocation } from "react-router-dom";
import type { Room } from "./RoomBox";
import { useEffect } from "react";
export default function RoomPage() {
  const location = useLocation();
  const room = location.state?.room as Room | undefined;
  useEffect(() => {
    if (!room) return;
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
            id: room.id,
            pop: true,
            password: "",
          }),
        });

        if (!response.ok) {
          throw new Error("Failed to join room");
        }
      } catch (err) {
        console.error(err);
      }
    };

    joinRoom();
  }, [room]); 

  return(
    <main className="home">
    <div className="grid-overlay">
      <GoBack />
      <h1> WELCOME TO THE {room?.name.toUpperCase()} ROOM</h1>
    </div>
    </main>
  );
}
