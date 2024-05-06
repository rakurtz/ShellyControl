import './Header.css';
import Clock from './clock/Clock';
import Weather from './weather/Weather';

export default function Header() {

    return (
    <div className="Header">
        <Clock/>
        <Weather />
    </div>
    );
}
  