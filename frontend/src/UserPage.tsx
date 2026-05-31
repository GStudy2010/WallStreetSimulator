import { useEffect, useState } from 'react';
import RoomCard from './RoomBox.tsx';
import type { Room } from './RoomBox.tsx'
import './UserPage.css';
import './HomePage.css';

interface QueryRoomResponse {
  rooms: Room[];
}

export default function UserPage() {
  const [rooms, setRooms] = useState<Room[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchRooms = async () => {
      try {
        const token = localStorage.getItem('authToken');

        const response = await fetch(
          '/api/query/rooms',
          {
            method: "GET",
            headers: {
              Authorization: `Bearer ${token}`,
            },
          }
        );

        if (!response.ok) {
          console.log(response.body);
          throw new Error('Failed to fetch rooms');
        }

        const data: QueryRoomResponse = await response.json();

        setRooms(data.rooms);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchRooms();
  }, []);

  if (loading) {
    return <h1>Loading...</h1>;
  }

  return (
    <div className="user-page">
      <div className="user-page-header">
        <h1>Trading Rooms</h1>
        <p>Join a simulation and compete with other traders.</p>
      </div>
      <div className="room-actions">
        <button
          type="submit"
          className="primary-btn"
        >
          Create an online Lobby
        </button>

        <button
          type="submit"
          className="primary-btn"
        >
          Create a private Lobby
        </button>
      </div>
      <div className="rooms-grid">
        {rooms.map((room) => (
          <RoomCard
            key={room.id}
            room={room}
          />
        ))}
      </div>
    </div>
  );
}
