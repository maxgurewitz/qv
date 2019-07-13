import React from 'react';
import { connect } from 'react-redux';
import { Dispatch } from 'redux';
import Grid from '@material-ui/core/Grid';
import Button from '@material-ui/core/Button';
import { Action } from './types';

interface LandingProps {
  login: () => void;
}

const Landing: React.FC<LandingProps> = (props) => {
  return (
    <Grid
      container
      direction="column"
      justify="center"
      alignItems="center"
      className="centered-grid">
      <div className="Landing">
        <Button onClick={props.login} variant="contained" color="primary" >
          Login
        </Button>
      </div>
    </Grid>
  );
}

function mapDispatchToProps(dispatch: Dispatch<Action>): LandingProps {
  return {
    login: () => {
      dispatch({
        source: 'internal',
        type: 'Login'
      })
    }
  };
}

export default connect(null, mapDispatchToProps)(Landing);