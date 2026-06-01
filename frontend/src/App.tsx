// import { useState } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './App.css'
import './index.css'
import HomePage from './HomePage'
import CreateAccount from './CreateAccount'
import LoginUser from './LoginUser'
import UserPage from './UserPage'
import CreatePrivate from './CreatePrivate'
import CreatePublic from './CreatePublic'
import RoomPage from './RoomPage'

function App() {

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/app" element={
          localStorage.getItem("authToken")
            ? <UserPage />
            : <HomePage />
        } />
        <Route path="/app/createAccount" element={<CreateAccount />}/>
        <Route path="/app/loginUser" element={<LoginUser />}/>
        <Route path="/app/createPrivate" element={<CreatePrivate />}/>
        <Route path="/app/createPublic" element={<CreatePublic />}/>
        <Route path="/app/room/:roomId" element={<RoomPage />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
