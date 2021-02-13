import { render, screen } from '@testing-library/react';
import App from './App';

describe('<App/>', () => {

  test('Main text rendered', () => {
    const consoleLog = console.log;
    console.log = jest.fn();

    render(<App />);

    expect(console.log).toHaveBeenCalled();

    const linkElement = screen.getByText('UNDER CONSTRUCTION');
    expect(linkElement).toBeInTheDocument();

    console.log = consoleLog;
  });

  test('mario!', () => {
    const consoleLog = console.log;
    console.log = jest.fn();

    render(<App />);
    expect(console.log).toHaveBeenCalled();

    console.log = consoleLog;
  });

});