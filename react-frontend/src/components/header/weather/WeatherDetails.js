import './WeatherDetails.css'

function FourHours(props) {
    let item = props.item;
    console.log("FourHours prop.item:", item);
    console.log(item.icon_url);

    return (
        <div className='forecast_item'> 
            <img className="weather_details_icon" src={item.icon} alt='weather icon'/>
            <ul>
                <li>{item.description}</li>
                <li>Real/Gefühlt: {item.temp.toFixed(1)}° / {item.feels_like.toFixed(1)}° C</li>
                <li>Windgeschwindindigkeit: {item.wind_speed.toFixed(1)} km/h</li>
            </ul>
        </div>
    )
}


export default function WeatherDetails(props) {
    const current = props.weather.current;
    const forecast = props.weather.forecast.forecast;


    const forecast_jsx = forecast.map((item) => 
        <div key={item.datetime}>
            <FourHours item={item}/>
        </div>
    );
    console.log(forecast_jsx);
    return (
        <div className='weather_details'>
            <h3>Aktuelles Wetter</h3>
            <div className='forecast_item'>
                <img className="weather_details_icon" src={current.icon_url} alt='weather icon'/>
                <ul>
                    <li>{current.description}</li>
                    <li>Real / Gefühlt: {current.temp} / {current.feels_like} °C</li>
                    <li>Windgeschwindindigkeit: {current.wind_speed} km/h</li>
                </ul>
            </div>
            <h3>Vorhersage (je +4h)</h3>
            {forecast_jsx}
        </div>
    )
}