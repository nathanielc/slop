import { useState } from "react";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Edit, Printer, BookmarkPlus, Globe, Lock, Download, X } from "lucide-react";
import { generateRecipeSvg } from "@/lib/wasm-recipe";

export interface RecipeViewProps {
  title: string;
  authorName: string;
  authorInitials: string;
  isPublic: boolean;
  tags: string[];
  content: string;
  isOwner?: boolean;
  onEdit?: () => void;
  onAddToBook?: () => void;
}

export function RecipeView({
  title,
  authorName,
  authorInitials,
  isPublic,
  tags,
  content,
  isOwner = false,
  onEdit,
  onAddToBook,
}: RecipeViewProps) {
  const [printModalOpen, setPrintModalOpen] = useState(false);
  const [svgContent, setSvgContent] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);

  const handlePrint = async () => {
    setIsGenerating(true);
    setPrintModalOpen(true);
    try {
      const fullRecipe = `# ${title}\n\n${content}`;
      const svg = await generateRecipeSvg(fullRecipe);
      setSvgContent(svg);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleDownload = () => {
    if (!svgContent) return;
    const blob = new Blob([svgContent], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${title.toLowerCase().replace(/\s+/g, "-")}.svg`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handlePrintSvg = () => {
    if (!svgContent) return;
    const printWindow = window.open("", "_blank");
    if (printWindow) {
      printWindow.document.write(`
        <!DOCTYPE html>
        <html>
          <head><title>${title}</title></head>
          <body style="margin:0;display:flex;justify-content:center;align-items:center;min-height:100vh;">
            ${svgContent}
          </body>
        </html>
      `);
      printWindow.document.close();
      printWindow.print();
    }
  };

  return (
    <>
      <Card className="w-full">
        <CardHeader className="space-y-4">
          <div className="flex flex-wrap items-start justify-between gap-4">
            <div className="space-y-2">
              <h1 className="font-serif text-3xl font-bold">{title}</h1>
              <div className="flex items-center gap-3">
                <Avatar className="h-8 w-8">
                  <AvatarFallback>{authorInitials}</AvatarFallback>
                </Avatar>
                <span className="text-muted-foreground">{authorName}</span>
                <Badge variant={isPublic ? "secondary" : "outline"}>
                  {isPublic ? (
                    <Globe className="h-3 w-3 mr-1" />
                  ) : (
                    <Lock className="h-3 w-3 mr-1" />
                  )}
                  {isPublic ? "Public" : "Private"}
                </Badge>
              </div>
            </div>
            <div className="flex flex-wrap gap-2">
              {isOwner && onEdit && (
                <Button variant="outline" onClick={onEdit} data-testid="button-edit">
                  <Edit className="h-4 w-4 mr-2" />
                  Edit
                </Button>
              )}
              {onAddToBook && (
                <Button variant="outline" onClick={onAddToBook} data-testid="button-add-to-book">
                  <BookmarkPlus className="h-4 w-4 mr-2" />
                  Add to Book
                </Button>
              )}
              <Button onClick={handlePrint} data-testid="button-print">
                <Printer className="h-4 w-4 mr-2" />
                Print
              </Button>
            </div>
          </div>
          {tags.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {tags.map((tag) => (
                <Badge key={tag} variant="outline" className="rounded-full">
                  {tag}
                </Badge>
              ))}
            </div>
          )}
        </CardHeader>
        <CardContent>
          <pre className="font-mono text-sm leading-relaxed whitespace-pre-wrap bg-muted/50 p-4 rounded-md">
            {content}
          </pre>
        </CardContent>
      </Card>

      <Dialog open={printModalOpen} onOpenChange={setPrintModalOpen}>
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-auto">
          <DialogHeader className="flex flex-row items-center justify-between gap-4">
            <DialogTitle>Print Preview</DialogTitle>
            <div className="flex gap-2">
              <Button
                variant="outline"
                onClick={handleDownload}
                disabled={!svgContent}
                data-testid="button-download-svg"
              >
                <Download className="h-4 w-4 mr-2" />
                Download SVG
              </Button>
              <Button
                onClick={handlePrintSvg}
                disabled={!svgContent}
                data-testid="button-print-svg"
              >
                <Printer className="h-4 w-4 mr-2" />
                Print
              </Button>
            </div>
          </DialogHeader>
          <div className="flex items-center justify-center p-4 bg-muted/30 rounded-md min-h-[400px]">
            {isGenerating ? (
              <div className="flex flex-col items-center gap-2">
                <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
                <span className="text-muted-foreground">Generating print view...</span>
              </div>
            ) : svgContent ? (
              <div
                dangerouslySetInnerHTML={{ __html: svgContent }}
                className="bg-white rounded shadow-sm"
              />
            ) : null}
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
