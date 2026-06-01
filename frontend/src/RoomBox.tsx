import './RoomBox.css';
import './HomePage.css';
import { useNavigate } from 'react-router-dom';
export interface Room {
  id: string;
  name: string;
  amount_players: number;
  current_players: number;
  start_cash: number;
  years: number;
}

interface RoomCardProps {
  room: Room;
}

export default function RoomCard({ room }: RoomCardProps) {
  const navigate = useNavigate();
  const handleJoin = () => {
    navigate(`/app/room/${room.id}`, {
      state: { room }
    });
  }
  return (
    <div className="room-card">
      <h3>{room.name}</h3>

      <div className="room-stats">
        <div className="room-stat">
          <span>Players</span>
          <span>{room.current_players}/{room.amount_players}</span>
        </div>

        <div className="room-stat">
          <span>Starting Cash</span>
          <span>${room.start_cash.toLocaleString()}</span>
        </div>

        <div className="room-stat">
          <span>Years</span>
          <span>{room.years}</span>
        </div>
      </div>

      <button
        className="join-btn"
        onClick={handleJoin}
      >
        Join Room
      </button>
    </div>
  );
}
