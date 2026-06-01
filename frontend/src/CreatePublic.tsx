import { useNavigate } from 'react-router-dom';
import './CreatePublic.css'
import GoBack from './GoBack'
import { useState } from 'react';
import Modal from './Modal';
export default function CreatePublic() {
  const navigate = useNavigate();

  const [name, setName] = useState("");
  const [maxPlayers, setMaxPlayers] = useState("");
  const [startCash, setStartCash] = useState("");
  const [length, setLength] = useState("");
  
  const [show400modal, setShow400modal] = useState(false);
  const [show500modal, setShow500modal] = useState(false);
  const [showCreatedModal, setShowCreateModal] = useState(false);
  const [showAcountCreatedmodal, setShowAcountCreatedmodal] = useState(false);
  const token = localStorage.getItem("authToken");
  const handleSubmit = async (
    e: React.FormEvent<HTMLFormElement>
  ) => {
    e.preventDefault();
    
    try {
      const response = await fetch("/api/createroom", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name: name,
          pop: true,
          all_players: Number(maxPlayers),
          start_cash: parseFloat(startCash),
          years: Number(length),
          password: "",
        }),
      });

      const data = await response.json();

      console.log(data);

      if (response.status === 400) {
        setShow400modal(true);
      }
      if (response.status === 500) {
        setShow500modal(true);
      }

      if (!response.ok) {
        throw new Error(data.message);
      }
      setShowCreateModal(true);

      navigate("/app");
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <main className="create-account-page">
      <div className="grid-overlay" />

      <div className="register-card">
        <div className="card-header">
          <GoBack />
          <h1>Create a Public game</h1>

          <p>
            Create a public game to play with other people and enjoy.
          </p>
        </div>

        <form
          className="register-form"
          onSubmit={handleSubmit}
        >
          <div className="input-group">
            <label>Name of the game</label>
            <input
              type="text"
              placeholder="Enter games name"
              value={name}
              onChange={(e) =>
                setName(e.target.value)
              }
            />
          </div>

          <div className="input-group">
            <label>Max amount of players</label>
            <input
              type="text"
              placeholder="Max amount of players ( 2-100 )"
              value={maxPlayers}
              onChange={(e) =>
                setMaxPlayers(e.target.value)
              }
            />
          </div>
          <div className="input-group">
            <label>Starting cash</label>
            <input
              type="text"
              placeholder="Starting cash ( thousands of dolars )"
              value={startCash}
              onChange={(e) =>
                setStartCash(e.target.value)
              }
            />
          </div>

          <div className="input-group">
            <label>Length of the game</label>
            <input
              type="text"
              placeholder="Length of the game ( years )"
              value={length}
              onChange={(e) =>
                setLength(e.target.value)
              }
            />
          </div>

          <button
            type="submit"
            className="create-btn"
          >
            Create the game
          </button>
        </form>
      </div>
        <Modal
          isOpen={show400modal}
          title="Email incorrect"
          message="Email you provided is incorrect"
          onClose={() => setShow400modal(false)}
        />
        <Modal
          isOpen={show500modal}
          title="Internal server error"
          message="Server had an issue processing your request"
          onClose={() => setShow500modal(false)}
        />
        <Modal
          isOpen={showCreatedModal}
          title="Created the public lobby"
          message="Public lobby created you can now join it on the main screen"
          onClose={() => setShowCreateModal(false)}
        />
        <Modal
          isOpen={showAcountCreatedmodal}
          title="Logged in to your account"
          message="You logged in, happy trading"
          onClose={() => {setShowAcountCreatedmodal(false);navigate("/app")}}
        />
    </main>
  );
}

