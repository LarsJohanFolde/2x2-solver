SELECT 
    CONCAT(scramble, ";"),
    CONCAT(c.name, ";"),
    CONCAT(s.competitionId, ";"),
    CONCAT(roundTypeId, ";"),
    CONCAT(groupId, ";"),
    CONCAT(isExtra, ";"),
    CONCAT(scrambleNum, ";"),
    c.countryId
FROM Scrambles s
JOIN Competitions c
  ON c.id = s.competitionId
WHERE c.results_posted_at IS NOT NULL AND eventId = '222'
ORDER BY DATE(results_posted_at) ASC;
