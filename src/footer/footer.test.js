import { render, screen } from '@testing-library/react';
import Footer from './footer';

describe('<Footer/>', () => {

  test('Copyright text renders correctly', () => {
    render(<Footer />);

    const date = new Date();
    const year = date.getFullYear();

    const element = screen.getByText(`Copyright ©${year}`);
    expect(element).toBeInTheDocument();
  });

})