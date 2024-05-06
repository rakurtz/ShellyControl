import CachedIcon from '@mui/icons-material/CachedOutlined';
import { Fab } from '@mui/material';


export default function ForceReload(props) {
    const setForceReload = props.setForceReload;

    const handleMouseUp = (e) => {
        console.log("in handleMouseUp()");
        e.preventDefault();
        setForceReload(true);
    };

    return (
        <Fab color='secondary' aria-label="forcereload"
            className='fab-content'
            onMouseUp={handleMouseUp} 
            onTouchEnd={handleMouseUp}
        >
            <CachedIcon fontSize="large"/>   
        </Fab>
    );
}