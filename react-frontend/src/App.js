import './App.css';
import { useEffect, useState } from 'react';

// Material UI Components
import {theme} from './theme'
import { ThemeProvider } from '@mui/material/styles';

// Own Components
import Header from './components/header/Header';
import LampContainer from './components/lamps/LampContainer';
import EditContainer from './components/configuration/ConfigContainer';
import Loading from './components/tools/Loading';
import FooterButtonsContainer from './components/footerButtons/FooterButtonsContainer';

// export
// export const backendUrl = `${process.env.REACT_APP_API_SERVER}`
export const backendUrl = window.location.origin + "/api"
export const reloadMilliSeconds = 20000 //`${process.env.PERIODICAL_RELOAD_SECONDS}` * 1000

export default function App() {
  const [lamps, setLamps] = useState([]);
  const [forceReload, setForceReload] = useState(true); 
  const [editMode, setEditMode] = useState(false);
  const [saveCommand, setSaveCommand] = useState(false);

  useEffect(() => {
    const intervalId = setInterval(() => { 
      console.log("in useEffect() of App.js: reloading lamps");
      fetch(backendUrl + "/get_lamps")
                .then(res => res.json())
                .then(data => setLamps(data))
                .catch(error => console.log('Error fetching data:', error));
      
    }, reloadMilliSeconds)
    return () => clearInterval(intervalId);
  }, []);

  useEffect(() => {
    console.log('INFO: forcereload changed to ' + forceReload);
    if (forceReload) {
      fetch(backendUrl + "/get_lamps_force_backend_sync")
                  .then(res => res.json())
                  .then(data => {
                    setLamps(data);
                    setForceReload(false);
                  })
                  .catch(error => console.log('Error fetching data:', error));
    }
  }, [forceReload]);


  // jsx parts
  const loading_jsx = <Loading/>;
  const lamps_jsx = <LampContainer lamps={lamps} setLamps={setLamps} />;
  const edit_jsx = <EditContainer setEditMode={setEditMode} saveCommand={saveCommand} setSaveCommand={setSaveCommand}/>; 
  const footer_buttons_jsx = (
    <FooterButtonsContainer 
        setForceReload={setForceReload} 
        lamps={lamps}
        setLamps={setLamps}
        editMode={editMode}
        setEditMode={setEditMode}
        setSaveCommand={setSaveCommand}
    />
  );

  // main return
  return (
    <ThemeProvider theme={theme}>
    <div className="App">
      <Header/>
      <div className='main-container'>
        {lamps.length === 0 && loading_jsx }
        {editMode ? edit_jsx : lamps_jsx}
        {lamps.length === 0 ? "" : footer_buttons_jsx}
      </div>
      <div className='bottom_space'/>
    </div>
  </ThemeProvider>
  )
  
}
