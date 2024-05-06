import './Weather.css';
import Popover from '@mui/material/Popover';
import AccessibilityNewOutlinedIcon from '@mui/icons-material/AccessibilityNewOutlined';

import { useEffect, useState } from "react";
import { backendUrl } from "../../../App";
import WeatherDetails from './WeatherDetails';


export default function Weather() {
    
    // weather data
    const [weather, setWeather] = useState(
        {
            current: { temp: 0.0, feels_like: 0.0, description: "no information yet", icon_url: "", wind_speed: 0.0},
            forecast: [],
        }
    );

    useEffect(() => {
        fetchWeather();
        const interval = setInterval(() => {
            fetchWeather()
        }, 1000 * 60); // 1 min
        return () => clearInterval(interval); // return a callback-function to be called on unMount() component
    }, []);

    const fetchWeather = () => {
        fetch(backendUrl + "/get_weather")
            .then(res => res.json())
            .then(data => setWeather(data))
            .catch(error => console.log('Error fetching data:', error));
        console.log("Update: Got weather information from server");
        console.log(weather);
    };

    // real_temp / feels_like
    const [showFeelsLike, setShowFeelsLike] = useState(false);
    const toggleShowFeelsLike = () => setShowFeelsLike(!showFeelsLike);
        
    const real_temp_jsx = <h3>{weather.current.temp.toFixed(1)} °C</h3>;
    const feels_like_jsx = <div className="weather_feels_like">
            <AccessibilityNewOutlinedIcon fontSize='small'/>
            &nbsp; &nbsp;
            <h3>{weather.current.feels_like.toFixed(1)} °C</h3>
        </div>;


    // Popover
    const [anchorEl, setAnchorEl] = useState(null);
    const handleClick = (event) => {
        setAnchorEl(event.currentTarget);
    };
    const handleClose = () => {
        setAnchorEl(null);
    };

    const open = Boolean(anchorEl);
    const id = open ? 'simple-popover' : undefined;

    return (
        <div className="weather">
            <div className="weather_temp_description" onClick={toggleShowFeelsLike}>
                <p>{weather.current.description}</p>
                {showFeelsLike ? feels_like_jsx : real_temp_jsx }
            </div>
            <div className='weather_icon_background'>
                <img className="weather_icon" src={weather.current.icon_url} onClick={handleClick} alt='weather icon'></img>
                <Popover
                    id={id}
                    open={open}
                    anchorEl={anchorEl}
                    onClose={handleClose}
                    anchorOrigin={{
                        vertical: 'bottom',
                        horizontal: 'right',
                    }}
                    transformOrigin={{
                        vertical: 'top',
                        horizontal: 'right',
                    }}
                >
                    <WeatherDetails weather={weather}/>
                </Popover>
            </div>
        </div>
    )

}