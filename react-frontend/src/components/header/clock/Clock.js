import './Clock.css';
import { useState } from "react";


export default function Clock() {
  
  const options_date = {
    day: '2-digit',
    month: 'long',
  };

  const options_time = {
    hour: '2-digit',
    minute:'2-digit',
  };

  let time =  new Date().toLocaleTimeString(['de-DE'], options_time);
  let date =  new Date().toLocaleDateString(['de-DE'], options_date);
  const [ctime, setTime] = useState(time)
  const [cdate, setDate] = useState(date)

  const updateTime = () =>{ 
    time =  new Date().toLocaleTimeString(['de-DE'], options_time);
    if (time !== ctime) {
        setTime(time)
    }
    date =  new Date().toLocaleDateString(['de-DE'], options_date);
    if (date !== cdate) {
        setDate(date)
    }
  }
  setInterval( updateTime, 1000 )
  return (
    <div className="clock">
      <p>{cdate}</p>
      <h3> {ctime} Uhr</h3>
    </div>
  );

}

