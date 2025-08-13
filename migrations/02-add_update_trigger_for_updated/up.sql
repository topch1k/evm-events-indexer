CREATE TRIGGER erc20_transfer_events_updated_trigger 
BEFORE UPDATE ON erc20_transfer_events 
BEGIN
	UPDATE  erc20_transfer_events 
	SET  updated  =  CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;