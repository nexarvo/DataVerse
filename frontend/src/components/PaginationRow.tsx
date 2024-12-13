/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';

interface PaginationRowProps {
  dataset: any[];
}

const PaginationRow: React.FC<PaginationRowProps> = ({ dataset }) => {
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize] = useState(20);
  const [totalRows, setTotalRows] = useState(dataset?.total_rows || 12);
  const goToPage = (page: number) => {
    if (page < 1 || page > Math.ceil(totalRows / pageSize)) return; // Prevent going out of bounds
    setCurrentPage(page);
  };

  const goToNextPage = () => {
    if (currentPage < Math.ceil(totalRows / pageSize)) {
      setCurrentPage(currentPage + 1);
    }
  };

  const goToPreviousPage = () => {
    if (currentPage > 1) {
      setCurrentPage(currentPage - 1);
    }
  };

  const goToFirstPage = () => {
    setCurrentPage(1);
  };

  const goToLastPage = () => {
    setCurrentPage(Math.ceil(totalRows / pageSize));
  };

  const generatePageNumbers = () => {
    const totalPages = Math.ceil(totalRows / pageSize);
    const maxVisiblePages = 5; // Max number of pages to display at once
    const pages = [];

    if (totalPages <= maxVisiblePages) {
      // If total pages are less than or equal to max visible pages, show all pages
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      // Show first page
      pages.push(1);

      // Show "..." for more pages
      if (currentPage > 3) {
        pages.push('...');
      }

      // Show current page and surrounding pages
      if (currentPage > 1) pages.push(currentPage - 1);
      pages.push(currentPage);
      if (currentPage < totalPages) pages.push(currentPage + 1);

      // Show last page
      if (currentPage < totalPages - 2) {
        pages.push('...');
      }
      pages.push(totalPages);
    }

    return pages;
  };
  return (
    <div className='pagination-controls'>
      <button onClick={goToFirstPage} disabled={currentPage === 1}>
        &lt;
      </button>
      {generatePageNumbers().map((page, index) => (
        <button
          key={index}
          onClick={() => typeof page === 'number' && goToPage(page)}
          disabled={typeof page === 'string'}
          className={`page-number ${currentPage === page ? 'active' : ''}`}
        >
          {page}
        </button>
      ))}
      <button
        onClick={goToLastPage}
        disabled={currentPage === Math.ceil(totalRows / pageSize)}
      >
        &gt;
      </button>
    </div>
  );
};

export default PaginationRow;
