import './Loading.css';
import CircularProgress from '@mui/material/CircularProgress';

export default function Loading() {
    return (
        <div className='main-container loading'>
            <h3>Loading</h3>
            <CircularProgress />
      </div>
    )
}