import './RoomBox.css';
import './HomePage.css';
export interface Room {
  id: string;
  name: string;
  amount_players: number;
  start_cash: number;
  years: number;
}

interface RoomCardProps {
  room: Room;
}

export default function RoomCard({ room }: RoomCardProps) {
  return (
    <div className="room-card">
      <h3>{room.name}</h3>

      <div className="room-stats">
        <div className="room-stat">
          <span>Players</span>
          <span>{room.amount_players}</span>
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
        onClick={() => console.log(`Joining room ${room.id}`)}
      >
        Join Room
      </button>
    </div>
  );
}
