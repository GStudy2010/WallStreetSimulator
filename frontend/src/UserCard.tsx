import './UserCard.css';

export interface User {
  id: string;
  name: string;
  typeof_user: boolean;
}

interface UserCardProps {
  user: User;
  position: number;
}

export default function UserCard({
  user,
  position,
}: UserCardProps) {
  return (
    <div className="user-card">
      <div className="user-rank">
        #{position}
      </div>

      <div className="user-info">
        <h3>{user.name}</h3>

        <span
          className={
            user.typeof_user
              ? "user-role"
              : "user-role admin"
          }
        >
          {user.typeof_user
            ? "Player"
            : "Host"}
        </span>
      </div>
    </div>
  );
}
