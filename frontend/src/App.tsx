// import { useState } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './App.css'
import './index.css'
import HomePage from './HomePage'
import CreateAccount from './CreateAccount'
import LoginUser from './LoginUser'
import UserPage from './UserPage'

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
      </Routes>
    </BrowserRouter>
  )
}

export default App
