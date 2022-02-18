import React, {useState} from 'react'
import axios from 'axios'
import {apiUrl} from './config'
import './App.css'

function App() {
  const [data, setData] = useState('요청 전')

  const clickPost = async () => {
    console.log(axios.post(`${apiUrl}/post_test`, {testKey: 'testValue'}))
    const res = await axios.post(`${apiUrl}/post_test`, {
      testKey: 'testValue',
    })
    setData(res.data)
  }

  return (
    <div className='App'>
      <button onClick={() => clickPost()}>Click Me</button>
      <p>{data}</p>
    </div>
  )
}

export default App
